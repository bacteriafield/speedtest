use crate::metrics::SpeedProgress;
use crate::server::Server;
use futures::StreamExt;
use reqwest::Client;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU64, Ordering},
};
use std::time::{Duration, Instant};
use tokio::sync::watch;

// 200 MB per stream request — large enough that we never hit the end
// before the duration timer fires.
const CHUNK_BYTES: u64 = 200_000_000;
// How often we sample throughput (500 ms → ~20 samples for a 10 s test)
const SAMPLE_INTERVAL_MS: u64 = 500;

pub async fn measure_download(
    client: &Client,
    server: &Server,
    duration: Duration,
    streams: usize,
    tx: watch::Sender<SpeedProgress>,
) -> f64 {
    let total_bytes = Arc::new(AtomicU64::new(0));
    let done = Arc::new(AtomicBool::new(false));

    // Spawn N parallel download tasks.
    let mut handles = Vec::with_capacity(streams);
    for _ in 0..streams {
        let client = client.clone();
        let url = format!("{}?bytes={}", server.download_url, CHUNK_BYTES);
        let counter = Arc::clone(&total_bytes);
        let stop_flag = Arc::clone(&done);

        handles.push(tokio::spawn(async move {
            loop {
                if stop_flag.load(Ordering::Relaxed) {
                    break;
                }

                let resp = match client.get(&url).send().await {
                    Ok(r) => r,
                    Err(_) => {
                        tokio::time::sleep(Duration::from_millis(100)).await;
                        continue;
                    }
                };

                let mut stream = resp.bytes_stream();
                while let Some(chunk) = stream.next().await {
                    if stop_flag.load(Ordering::Relaxed) {
                        break;
                    }
                    if let Ok(b) = chunk {
                        counter.fetch_add(b.len() as u64, Ordering::Relaxed);
                    }
                }
            }
        }));
    }

    // Sampling loop — runs for `duration`, sending progress updates.
    let start = Instant::now();
    let mut prev_bytes: u64 = 0;
    let mut prev_ts = start;
    let mut history: Vec<f64> = Vec::new();
    let mut peak = 0.0_f64;

    while start.elapsed() < duration {
        tokio::time::sleep(Duration::from_millis(SAMPLE_INTERVAL_MS)).await;

        let now = Instant::now();
        let curr_bytes = total_bytes.load(Ordering::Relaxed);
        let delta_b = curr_bytes.saturating_sub(prev_bytes);
        let delta_s = now.duration_since(prev_ts).as_secs_f64().max(1e-9);
        let mbps = (delta_b as f64 * 8.0) / delta_s / 1_000_000.0;

        peak = peak.max(mbps);
        history.push(mbps);

        let avg = if history.is_empty() {
            0.0
        } else {
            history.iter().sum::<f64>() / history.len() as f64
        };

        let _ = tx.send(SpeedProgress {
            current_mbps: mbps,
            avg_mbps: avg,
            peak_mbps: peak,
            elapsed_secs: start.elapsed().as_secs_f64(),
            history: history.clone(),
        });

        prev_bytes = curr_bytes;
        prev_ts = now;
    }

    // Signal all tasks to stop.
    done.store(true, Ordering::Relaxed);
    for h in handles {
        let _ = h.await;
    }

    // Return trimmed mean (drop first and last samples — they're often noisy).
    let trimmed = if history.len() > 4 {
        &history[1..history.len() - 1]
    } else {
        history.as_slice()
    };

    if trimmed.is_empty() {
        0.0
    } else {
        trimmed.iter().sum::<f64>() / trimmed.len() as f64
    }
}
