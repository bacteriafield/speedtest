use reqwest::Client;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Server {
    pub id: &'static str,
    pub name: &'static str,
    pub location: &'static str,
    pub download_url: &'static str,
    pub upload_url: &'static str,
    pub ping_url: &'static str,
}

// Cloudflare's global CDN auto-routes to the nearest PoP — ideal as default.
// Additional entries demonstrate the extensible list pattern.
pub static SERVERS: &[Server] = &[
    Server {
        id: "cloudflare",
        name: "Cloudflare Speed",
        location: "Global CDN",
        download_url: "https://speed.cloudflare.com/__down",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://speed.cloudflare.com/__down?bytes=1",
    },
    Server {
        id: "nbn-sydney",
        name: "Cloudflare Sydney",
        location: "Sydney, AU",
        download_url: "https://speed.cloudflare.com/__down",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://speed.cloudflare.com/__down?bytes=1",
    },
];

// ── Server selection ──────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum SelectionError {
    #[error("server '{0}' not found")]
    NotFound(String),
    #[error("no reachable server found")]
    NoneReachable,
    #[error("request failed: {0}")]
    Http(#[from] reqwest::Error),
}

/// Returns the server with the lowest measured latency.
/// If `override_id` is given, skip probing and return that server directly.
pub async fn select_best(
    client: &Client,
    override_id: Option<&str>,
) -> Result<&'static Server, SelectionError> {
    if let Some(id) = override_id {
        return SERVERS
            .iter()
            .find(|s| s.id == id)
            .ok_or_else(|| SelectionError::NotFound(id.to_owned()));
    }

    // Probe all servers in parallel and pick the fastest.
    let probes = SERVERS.iter().map(|s| probe(client, s));
    let results = futures::future::join_all(probes).await;

    results
        .into_iter()
        .zip(SERVERS.iter())
        .filter_map(|(r, s)| r.ok().map(|ms| (ms, s)))
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .map(|(_, s)| s)
        .ok_or(SelectionError::NoneReachable)
}

/// Measures average RTT (ms) to a server using a few small HEAD requests.
pub async fn probe(client: &Client, server: &Server) -> Result<f64, reqwest::Error> {
    const PROBES: u32 = 3;
    let mut total = 0u128;

    for _ in 0..PROBES {
        let start = Instant::now();
        client
            .head(server.ping_url)
            .timeout(Duration::from_secs(5))
            .send()
            .await?;
        total += start.elapsed().as_millis();
    }

    Ok(total as f64 / PROBES as f64)
}
