use reqwest::{Client, ClientBuilder};
use std::time::Duration;

/// Builds a general-purpose reqwest client (auth + JSON headers).
pub fn build_client() -> Client {
    ClientBuilder::new()
        .user_agent(concat!("speedtest-rs/", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(30))
        .tcp_nodelay(true)
        .pool_max_idle_per_host(16)
        .build()
        .expect("failed to build HTTP client")
}

/// Builds a high-throughput download/upload client (no per-request timeout,
/// larger connection pool).
pub fn build_transfer_client() -> Client {
    ClientBuilder::new()
        .user_agent(concat!("speedtest-rs/", env!("CARGO_PKG_VERSION")))
        .connect_timeout(Duration::from_secs(10))
        .tcp_nodelay(true)
        .pool_max_idle_per_host(32)
        .build()
        .expect("failed to build transfer client")
}
