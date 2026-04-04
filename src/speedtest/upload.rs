use crate::metrics::SpeedProgress;
use crate::server::Server;
use reqwest::Client;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU64, Ordering},
};
use std::time::{Duration, Instant};
use tokio::sync::watch;

// 10 MB payload per request — enough for accurate measurement without
// overwhelming memory.
const PAYLOAD_BYTES: usize = 10_000_000;
const SAMPLE_INTERVAL_MS: u64 = 500;

pub async fn measure_upload(
    client: &Client,
    server: &Server,
    duration: Duration,
    streams: usize,
    tx: watch::Sender<SpeedProgress>,
) -> f64 {
    let total_bytes = Arc::new(AtomicU64::new(0));
    let done = Arc::new(AtomicBool::new(false));

    // Pre-build the payload once and share it.
    let payload: Arc<Vec<u8>> = Arc::new(vec![0u8; PAYLOAD_BYTES]);

    let mut handles = Vec::with_capacity(streams);
    for _ in 0..streams {
        let client = client.clone();
        let url = server.upload_url.to_owned();
        let counter = Arc::clone(&total_bytes);
        let stop_flag = Arc::clone(&done);
        let data = Arc::clone(&payload);

        handles.push(tokio::spawn(async move {
            loop {
                if stop_flag.load(Ordering::Relaxed) {
                    break;
                }

                let body_bytes = data.as_ref().clone();
                let len = body_bytes.len() as u64;

                let result = client
                    .post(&url)
                    .header("Content-Type", "application/octet-stream")
                    .body(body_bytes)
                    .send()
                    .await;

                match result {
                    Ok(_) => {
                        counter.fetch_add(len, Ordering::Relaxed);
                    }
                    Err(_) => {
                        tokio::time::sleep(Duration::from_millis(100)).await;
                    }
                }
            }
        }));
    }

    // Sampling loop — mirrors the download sampler.
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

    done.store(true, Ordering::Relaxed);
    for h in handles {
        let _ = h.await;
    }

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
