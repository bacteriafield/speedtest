use crate::metrics::TestResult;
use serde::Serialize;
use std::{fs, path::Path};

#[derive(Serialize)]
pub struct JsonOutput<'a> {
    pub server: ServerJson<'a>,
    pub ping_ms: f64,
    pub jitter_ms: f64,
    pub download_mbps: f64,
    pub upload_mbps: f64,
    pub timestamp: &'a str,
}

#[derive(Serialize)]
pub struct ServerJson<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

impl<'a> From<&'a TestResult> for JsonOutput<'a> {
    fn from(r: &'a TestResult) -> Self {
        Self {
            server: ServerJson {
                id: &r.server_id,
                name: &r.server_name,
            },
            ping_ms: r.ping.avg_ms,
            jitter_ms: r.ping.jitter_ms,
            download_mbps: r.download_mbps,
            upload_mbps: r.upload_mbps,
            timestamp: &r.timestamp,
        }
    }
}

pub fn to_json(result: &TestResult) -> String {
    let out = JsonOutput::from(result);
    serde_json::to_string_pretty(&out).expect("serialize json")
}

pub fn export(result: &TestResult, path: &str) -> std::io::Result<()> {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("json")
        .to_lowercase();

    let content = match ext.as_str() {
        "csv" => to_csv(result),
        _ => to_json(result),
    };

    fs::write(path, content)?;
    eprintln!("Results exported to {}", path);
    Ok(())
}

fn to_csv(r: &TestResult) -> String {
    let header = "timestamp,server_id,server_name,ping_ms,jitter_ms,download_mbps,upload_mbps";
    let row = format!(
        "{},{},{},{:.2},{:.2},{:.2},{:.2}",
        r.timestamp,
        r.server_id,
        r.server_name,
        r.ping.avg_ms,
        r.ping.jitter_ms,
        r.download_mbps,
        r.upload_mbps,
    );
    format!("{}\n{}\n", header, row)
}

pub fn print_summary(result: &TestResult) {
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!(
        "  RESULTS  │  {}  ({})",
        result.server_name, result.timestamp
    );
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!(
        "  Ping        {:>8.1} ms     jitter  {:.1} ms",
        result.ping.avg_ms, result.ping.jitter_ms
    );
    println!("  Download    {:>8.1} Mbps", result.download_mbps);
    println!("  Upload      {:>8.1} Mbps", result.upload_mbps);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
}
