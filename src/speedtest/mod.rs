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

    let renderer_handle = if !cli.json {
        let rx = state_rx.clone();
        let server_label = format!("{} ({})", server.name, server.location);
        Some(tokio::spawn(async move {
            let mut renderer = LiveRenderer::new(server_label);
            renderer.run(rx).await;
        }))
    } else {
        None
    };

    let probe_client = crate::network::build_client();

    // Ping
    send_phase(&state_tx, Phase::Ping);
    let ping = ping::measure_ping(&probe_client, server, cli.ping_count).await;
    send_state(&state_tx, |s| s.ping = Some(ping.clone()));

    // Download
    let download_mbps = if !cli.no_download {
        let (dl_tx, _dl_rx) = watch::channel(SpeedProgress::default());
        send_phase(&state_tx, Phase::Download);
        forward_progress(&state_tx, _dl_rx, |s, p| s.download = Some(p));
        download::measure_download(&transfer_client, server, duration, cli.streams, dl_tx).await
    } else {
        0.0
    };

    // Upload
    let upload_mbps = if !cli.no_upload {
        let (ul_tx, _ul_rx) = watch::channel(SpeedProgress::default());
        send_phase(&state_tx, Phase::Upload);
        forward_progress(&state_tx, _ul_rx, |s, p| s.upload = Some(p));
        upload::measure_upload(&transfer_client, server, duration, cli.streams, ul_tx).await
    } else {
        0.0
    };

    send_phase(&state_tx, Phase::Done);

    // Aguarda o renderer terminar de verdade antes de continuar
    if let Some(handle) = renderer_handle {
        let _ = handle.await;
    }

    TestResult {
        server_name: server.name.to_owned(),
        server_id: server.id.to_owned(),
        ping,
        download_mbps,
        upload_mbps,
        timestamp: Utc::now().to_rfc3339(),
    }
}

fn send_phase(tx: &watch::Sender<LiveState>, phase: Phase) {
    let mut s = tx.borrow().clone();
    s.phase = phase;
    let _ = tx.send(s);
}

fn send_state(tx: &watch::Sender<LiveState>, f: impl FnOnce(&mut LiveState)) {
    let mut s = tx.borrow().clone();
    f(&mut s);
    let _ = tx.send(s);
}

fn forward_progress(
    state_tx: &watch::Sender<LiveState>,
    mut rx: watch::Receiver<SpeedProgress>,
    update: impl Fn(&mut LiveState, SpeedProgress) + Send + 'static,
) {
    let tx2 = state_tx.clone();
    tokio::spawn(async move {
        loop {
            if rx.changed().await.is_err() {
                break;
            }
            let prog = rx.borrow().clone();
            let mut s = tx2.borrow().clone();
            update(&mut s, prog);
            let _ = tx2.send(s);
        }
    });
}
