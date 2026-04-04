use reqwest::{Client, ClientBuilder};
use std::time::Duration;

/// Builds a general-purpose reqwest client (auth + JSON headers).
pub fn build_client() -> Client {
    ClientBuilder::new()
        .user_agent(concat!("speedtest-rs/", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(30))
        .tcp_nodelay(true)
        .pool_max_idle_per_host(16)
        .http1_only()
        .build()
        .expect("failed to build HTTP client")
}

/// Builds a high-throughput download/upload client (no per-request timeout,
/// larger connection pool).
pub fn build_transfer_client() -> Client {
    Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .timeout(Duration::from_secs(60))
        .connect_timeout(Duration::from_secs(10))
        .tcp_keepalive(Some(Duration::from_secs(30)))
        .pool_max_idle_per_host(16)
        .pool_idle_timeout(Duration::from_secs(90))
        .no_gzip()
        .no_brotli()
        .no_deflate()
        .http1_only()
        .build()
        .expect("Failed to build HTTP client")
}
