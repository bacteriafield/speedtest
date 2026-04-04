use reqwest::Client;
use serde::Deserialize;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Server {
    pub id: &'static str,
    pub name: &'static str,
    pub location: &'static str,
    pub country: &'static str,
    pub lat: f64,
    pub lon: f64,
    pub download_url: &'static str,
    pub upload_url: &'static str,
    pub ping_url: &'static str,
}

// Cloudflare's global CDN auto-routes to the nearest PoP — ideal as default.
// Additional entries demonstrate the extensible list pattern.
pub static SERVERS: &[Server] = &[
    // South America
    Server {
        id: "cf-sao-paulo",
        name: "Cloudflare São Paulo",
        location: "São Paulo, BR",
        country: "BR",
        lat: -23.55,
        lon: -46.63,
        download_url: "https://speed.cloudflare.com/__down",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://speed.cloudflare.com/__down?bytes=1",
    },
    Server {
        id: "bunny-sao-paulo",
        name: "Bunny CDN São Paulo",
        location: "São Paulo, BR",
        country: "BR",
        lat: -23.55,
        lon: -46.63,
        download_url: "https://sao.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://sao.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-buenos-aires",
        name: "Bunny CDN Buenos Aires",
        location: "Buenos Aires, AR",
        country: "AR",
        lat: -34.60,
        lon: -58.38,
        download_url: "https://bue.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://bue.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-bogota",
        name: "Bunny CDN Bogotá",
        location: "Bogotá, CO",
        country: "CO",
        lat: 4.71,
        lon: -74.07,
        download_url: "https://bog.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://bog.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-santiago",
        name: "Bunny CDN Santiago",
        location: "Santiago, CL",
        country: "CL",
        lat: -33.45,
        lon: -70.67,
        download_url: "https://scl.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://scl.speedtest.bunny.net/10mb.bin",
    },
    // North America
    Server {
        id: "cf-new-york",
        name: "Cloudflare New York",
        location: "New York, US",
        country: "US",
        lat: 40.71,
        lon: -74.01,
        download_url: "https://speed.cloudflare.com/__down",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://speed.cloudflare.com/__down?bytes=1",
    },
    Server {
        id: "bunny-los-angeles",
        name: "Bunny CDN Los Angeles",
        location: "Los Angeles, US",
        country: "US",
        lat: 34.05,
        lon: -118.24,
        download_url: "https://lax.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://lax.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-miami",
        name: "Bunny CDN Miami",
        location: "Miami, US",
        country: "US",
        lat: 25.77,
        lon: -80.19,
        download_url: "https://mia.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://mia.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-chicago",
        name: "Bunny CDN Chicago",
        location: "Chicago, US",
        country: "US",
        lat: 41.88,
        lon: -87.63,
        download_url: "https://chi.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://chi.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-toronto",
        name: "Bunny CDN Toronto",
        location: "Toronto, CA",
        country: "CA",
        lat: 43.65,
        lon: -79.38,
        download_url: "https://tor.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://tor.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-mexico-city",
        name: "Bunny CDN Mexico City",
        location: "Mexico City, MX",
        country: "MX",
        lat: 19.43,
        lon: -99.13,
        download_url: "https://mex.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://mex.speedtest.bunny.net/10mb.bin",
    },
    // Europe
    Server {
        id: "bunny-london",
        name: "Bunny CDN London",
        location: "London, GB",
        country: "GB",
        lat: 51.51,
        lon: -0.13,
        download_url: "https://lon.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://lon.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-frankfurt",
        name: "Bunny CDN Frankfurt",
        location: "Frankfurt, DE",
        country: "DE",
        lat: 50.11,
        lon: 8.68,
        download_url: "https://fra.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://fra.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-amsterdam",
        name: "Bunny CDN Amsterdam",
        location: "Amsterdam, NL",
        country: "NL",
        lat: 52.37,
        lon: 4.90,
        download_url: "https://ams.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://ams.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-paris",
        name: "Bunny CDN Paris",
        location: "Paris, FR",
        country: "FR",
        lat: 48.85,
        lon: 2.35,
        download_url: "https://par.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://par.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-stockholm",
        name: "Bunny CDN Stockholm",
        location: "Stockholm, SE",
        country: "SE",
        lat: 59.33,
        lon: 18.07,
        download_url: "https://sto.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://sto.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-warsaw",
        name: "Bunny CDN Warsaw",
        location: "Warsaw, PL",
        country: "PL",
        lat: 52.23,
        lon: 21.01,
        download_url: "https://waw.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://waw.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-madrid",
        name: "Bunny CDN Madrid",
        location: "Madrid, ES",
        country: "ES",
        lat: 40.42,
        lon: -3.70,
        download_url: "https://mad.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://mad.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-milan",
        name: "Bunny CDN Milan",
        location: "Milan, IT",
        country: "IT",
        lat: 45.46,
        lon: 9.19,
        download_url: "https://mxp.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://mxp.speedtest.bunny.net/10mb.bin",
    },
    // Africa
    Server {
        id: "bunny-johannesburg",
        name: "Bunny CDN Johannesburg",
        location: "Johannesburg, ZA",
        country: "ZA",
        lat: -26.20,
        lon: 28.04,
        download_url: "https://jnb.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://jnb.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-nairobi",
        name: "Bunny CDN Nairobi",
        location: "Nairobi, KE",
        country: "KE",
        lat: -1.29,
        lon: 36.82,
        download_url: "https://nbo.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://nbo.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-lagos",
        name: "Bunny CDN Lagos",
        location: "Lagos, NG",
        country: "NG",
        lat: 6.45,
        lon: 3.40,
        download_url: "https://lag.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://lag.speedtest.bunny.net/10mb.bin",
    },
    // Middle East
    Server {
        id: "bunny-dubai",
        name: "Bunny CDN Dubai",
        location: "Dubai, AE",
        country: "AE",
        lat: 25.20,
        lon: 55.27,
        download_url: "https://dxb.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://dxb.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-istanbul",
        name: "Bunny CDN Istanbul",
        location: "Istanbul, TR",
        country: "TR",
        lat: 41.01,
        lon: 28.95,
        download_url: "https://ist.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://ist.speedtest.bunny.net/10mb.bin",
    },
    // Asia
    Server {
        id: "bunny-singapore",
        name: "Bunny CDN Singapore",
        location: "Singapore, SG",
        country: "SG",
        lat: 1.35,
        lon: 103.82,
        download_url: "https://sin.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://sin.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-tokyo",
        name: "Bunny CDN Tokyo",
        location: "Tokyo, JP",
        country: "JP",
        lat: 35.68,
        lon: 139.69,
        download_url: "https://tyo.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://tyo.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-mumbai",
        name: "Bunny CDN Mumbai",
        location: "Mumbai, IN",
        country: "IN",
        lat: 19.08,
        lon: 72.88,
        download_url: "https://bom.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://bom.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-seoul",
        name: "Bunny CDN Seoul",
        location: "Seoul, KR",
        country: "KR",
        lat: 37.57,
        lon: 126.98,
        download_url: "https://icn.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://icn.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-hongkong",
        name: "Bunny CDN Hong Kong",
        location: "Hong Kong, HK",
        country: "HK",
        lat: 22.32,
        lon: 114.17,
        download_url: "https://hkg.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://hkg.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-bangalore",
        name: "Bunny CDN Bangalore",
        location: "Bangalore, IN",
        country: "IN",
        lat: 12.97,
        lon: 77.59,
        download_url: "https://blr.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://blr.speedtest.bunny.net/10mb.bin",
    },
    // Oceania
    Server {
        id: "bunny-sydney",
        name: "Bunny CDN Sydney",
        location: "Sydney, AU",
        country: "AU",
        lat: -33.87,
        lon: 151.21,
        download_url: "https://syd.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://syd.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-melbourne",
        name: "Bunny CDN Melbourne",
        location: "Melbourne, AU",
        country: "AU",
        lat: -37.81,
        lon: 144.96,
        download_url: "https://mel.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://mel.speedtest.bunny.net/10mb.bin",
    },
    Server {
        id: "bunny-auckland",
        name: "Bunny CDN Auckland",
        location: "Auckland, NZ",
        country: "NZ",
        lat: -36.87,
        lon: 174.77,
        download_url: "https://akl.speedtest.bunny.net/10mb.bin",
        upload_url: "https://speed.cloudflare.com/__up",
        ping_url: "https://akl.speedtest.bunny.net/10mb.bin",
    },
];

#[derive(Deserialize)]
struct CfMeta {
    latitude: Option<f64>,
    longitude: Option<f64>,
    country: Option<String>,
    city: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserLocation {
    pub lat: f64,
    pub lon: f64,
    pub country: String,
    pub city: String,
}

pub async fn locate_user(client: &Client) -> Option<UserLocation> {
    let meta: CfMeta = client
        .get("https://speed.cloudflare.com/meta")
        .timeout(Duration::from_secs(5))
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;

    Some(UserLocation {
        lat: meta.latitude?,
        lon: meta.longitude?,
        country: meta.country.unwrap_or_default(),
        city: meta.city.unwrap_or_default(),
    })
}

// Haversine distance (km)
pub fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6_371.0;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    2.0 * R * a.sqrt().asin()
}

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
) -> Result<(&'static Server, UserLocation), SelectionError> {
    if let Some(id) = override_id {
        let server = SERVERS
            .iter()
            .find(|s| s.id == id)
            .ok_or_else(|| SelectionError::NotFound(id.to_owned()))?;
        let loc = locate_user(client).await.unwrap_or(UserLocation {
            lat: 0.0,
            lon: 0.0,
            country: "?".into(),
            city: "?".into(),
        });
        return Ok((server, loc));
    }

    // Geolocate
    let user = locate_user(client).await.unwrap_or(UserLocation {
        lat: -23.55,
        lon: -46.63, // fallback: São Paulo
        country: "??".into(),
        city: "unknown".into(),
    });

    // Sort by distance
    let mut ranked: Vec<(&'static Server, f64)> = SERVERS
        .iter()
        .map(|s| {
            let km = haversine_km(user.lat, user.lon, s.lat, s.lon);
            (s, km)
        })
        .collect();
    ranked.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let candidates = &ranked[..ranked.len().min(3)];
    let results = futures::future::join_all(candidates.iter().map(|(s, _)| probe(client, s))).await;

    let best = results
        .into_iter()
        .zip(candidates.iter())
        .filter_map(|(r, (s, _))| r.ok().map(|ms| (ms, *s)))
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .map(|(_, s)| s)
        .ok_or(SelectionError::NoneReachable)?;

    Ok((best, user))
}

// Average RTT to a server over 3 HEAD probes.
pub async fn probe(client: &Client, server: &Server) -> Result<f64, reqwest::Error> {
    let mut total = 0u128;
    for _ in 0..3 {
        let t = Instant::now();
        client.head(server.ping_url).timeout(Duration::from_secs(5)).send().await?;
        total += t.elapsed().as_millis();
    }
    Ok(total as f64 / 3.0)
}
