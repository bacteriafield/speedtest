use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "speedtest",
    about = "Network speed test — download, upload, ping, jitter",
    version
)]
pub struct Cli {
    /// Override server (by ID from --list-servers)
    #[arg(long, short)]
    pub server: Option<String>,

    /// Output results as JSON (suppresses live UI)
    #[arg(long)]
    pub json: bool,

    /// Test duration in seconds (per phase)
    #[arg(long, default_value_t = 10)]
    pub duration: u64,

    /// Number of parallel download/upload streams
    #[arg(long, default_value_t = 4)]
    pub streams: usize,

    /// Number of ping samples
    #[arg(long, default_value_t = 20)]
    pub ping_count: u32,

    /// Export results to file (e.g. results.json or results.csv)
    #[arg(long)]
    pub export: Option<String>,

    /// Skip download test
    #[arg(long)]
    pub no_download: bool,

    /// Skip upload test
    #[arg(long)]
    pub no_upload: bool,

    /// List available servers and exit
    #[arg(long)]
    pub list_servers: bool,
}
