use crate::metrics::PingResult;
use crate::server::Server;
use reqwest::Client;
use std::time::{Duration, Instant};

pub async fn measure_ping(client: &Client, server: &Server, count: u32) -> PingResult {
    let mut samples = Vec::with_capacity(count as usize);

    for _ in 0..count {
        let start = Instant::now();
        let _ = client.head(server.ping_url).timeout(Duration::from_secs(5)).send().await;
        let ms = start.elapsed().as_secs_f64() * 1000.0;
        samples.push(ms);

        // Small gap between pings to avoid burst-induced bias.
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    if samples.is_empty() {
        return PingResult::default();
    }

    let min = samples.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = samples.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let avg = samples.iter().sum::<f64>() / samples.len() as f64;

    let jitter = if samples.len() > 1 {
        let diffs: f64 = samples.windows(2).map(|w| (w[1] - w[0]).abs()).sum();
        diffs / (samples.len() - 1) as f64
    } else {
        0.0
    };

    PingResult { avg_ms: avg, min_ms: min, max_ms: max, jitter_ms: jitter }
}
