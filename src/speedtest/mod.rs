pub mod download;
pub mod ping;
pub mod upload;

use crate::cli::Cli;
use crate::metrics::{PingResult, SpeedProgress, TestResult};
use crate::network::build_transfer_client;
use crate::renderer::LiveRenderer;
use crate::server::Server;
use chrono::Utc;
use std::time::Duration;
use tokio::sync::watch;

#[derive(Debug, Clone, Default)]
pub struct LiveState {
    pub phase: Phase,
    pub ping: Option<PingResult>,
    pub download: Option<SpeedProgress>,
    pub upload: Option<SpeedProgress>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Phase {
    #[default]
    Idle,
    Ping,
    Download,
    Upload,
    Done,
}

pub async fn run(cli: &Cli, server: &Server) -> TestResult {
    let transfer_client = build_transfer_client();
    let duration = Duration::from_secs(cli.duration);

    let (state_tx, state_rx) = watch::channel(LiveState::default());

    if !cli.json {
        let rx = state_rx.clone();
        let server_label = format!("{} ({})", server.name, server.location);
        tokio::spawn(async move {
            let mut renderer = LiveRenderer::new(server_label);
            renderer.run(rx).await;
        });
    }

    let probe_client = crate::network::build_client();

    {
        let mut s = state_tx.borrow().clone();
        s.phase = Phase::Ping;
        let _ = state_tx.send(s);
    }

    let ping = ping::measure_ping(&probe_client, server, cli.ping_count).await;

    {
        let mut s = state_tx.borrow().clone();
        s.ping = Some(ping.clone());
        let _ = state_tx.send(s);
    }

    let download_mbps = if !cli.no_download {
        let (dl_tx, dl_rx) = watch::channel(SpeedProgress::default());

        {
            let mut s = state_tx.borrow().clone();
            s.phase = Phase::Download;
            let _ = state_tx.send(s);
        }

        // Forward download progress to main state.
        {
            let state_tx2 = state_tx.clone();
            let mut rx2 = dl_rx.clone();
            tokio::spawn(async move {
                loop {
                    if rx2.changed().await.is_err() {
                        break;
                    }
                    let prog = rx2.borrow().clone();
                    let mut s = state_tx2.borrow().clone();
                    s.download = Some(prog);
                    let _ = state_tx2.send(s);
                }
            });
        }

        download::measure_download(&transfer_client, server, duration, cli.streams, dl_tx).await
    } else {
        0.0
    };

    let upload_mbps = if !cli.no_upload {
        let (ul_tx, ul_rx) = watch::channel(SpeedProgress::default());

        {
            let mut s = state_tx.borrow().clone();
            s.phase = Phase::Upload;
            let _ = state_tx.send(s);
        }

        {
            let state_tx2 = state_tx.clone();
            let mut rx2 = ul_rx.clone();
            tokio::spawn(async move {
                loop {
                    if rx2.changed().await.is_err() {
                        break;
                    }
                    let prog = rx2.borrow().clone();
                    let mut s = state_tx2.borrow().clone();
                    s.upload = Some(prog);
                    let _ = state_tx2.send(s);
                }
            });
        }

        upload::measure_upload(&transfer_client, server, duration, cli.streams, ul_tx).await
    } else {
        0.0
    };

    {
        let mut s = state_tx.borrow().clone();
        s.phase = Phase::Done;
        let _ = state_tx.send(s);
    }

    // Give the renderer one final paint.
    tokio::time::sleep(Duration::from_millis(150)).await;

    TestResult {
        server_name: server.name.to_owned(),
        server_id: server.id.to_owned(),
        ping: ping,
        download_mbps: download_mbps,
        upload_mbps: upload_mbps,
        timestamp: Utc::now().to_rfc3339(),
    }
}
