#[allow(dead_code)]
#[allow(unused)]
pub mod chart;

use crate::helpers::render::{render_phase_body, render_phase_header};
use crate::speedtest::{LiveState, Phase};
use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    style::{Color, ResetColor, SetForegroundColor, Stylize},
    terminal,
};
use std::io::{Write, stdout};
use std::time::Duration;
use tokio::sync::watch;

pub const CHART_WIDTH: usize = 52;
pub const CHART_HEIGHT: usize = 5;
// Total number of lines the live block occupies (we move back up this many).
pub const LIVE_LINES: u16 = 26;

pub struct LiveRenderer {
    server_label: String,
    lines_printed: u16,
}

impl LiveRenderer {
    pub fn new(server_label: String) -> Self {
        Self { server_label, lines_printed: 0 }
    }

    /// Main render loop — redraws at ~10 fps until Phase::Done.
    pub async fn run(&mut self, mut rx: watch::Receiver<LiveState>) {
        // Initial blank frame so there's something to overwrite.
        self.draw(&LiveState::default());

        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;

            if rx.has_changed().unwrap_or(false) {
                let _ = rx.changed().await;
            }

            let state = rx.borrow().clone();
            let done = state.phase == Phase::Done;
            self.draw(&state);

            if done {
                break;
            }
        }

        // Move cursor below the drawn block so subsequent output is clean.
        let _ = stdout().execute(cursor::MoveDown(2));
    }

    fn draw(&mut self, state: &LiveState) {
        let mut out = stdout();

        // Move back up to overwrite previous frame (skip on first paint).
        if self.lines_printed > 0 {
            let _ = out.queue(cursor::MoveToPreviousLine(self.lines_printed));
        }

        let mut lines: Vec<String> = Vec::new();

        lines.push(format!("{}", format!(" ◆ speedtest  │  {}", self.server_label).bold()));
        lines.push(format!("{}", "─".repeat(64)));

        match &state.ping {
            Some(p) => {
                lines.push(format!(
                    "  PING     {:>7.1} ms   min {:>5.1}  max {:>5.1}  jitter {:>4.1} ms",
                    p.avg_ms, p.min_ms, p.max_ms, p.jitter_ms
                ));
            },
            None => {
                let label = if state.phase == Phase::Ping { "measuring…" } else { "pending" };
                lines.push(format!("  PING     {}", label));
            },
        }

        lines.push("─".repeat(64));
        lines.push(render_phase_header(
            "DOWNLOAD",
            &state.download,
            state.phase == Phase::Download,
        ));
        render_phase_body(&mut lines, &state.download);
        lines.push("─".repeat(64));
        lines.push(render_phase_header("UPLOAD  ", &state.upload, state.phase == Phase::Upload));
        render_phase_body(&mut lines, &state.upload);
        lines.push("─".repeat(64));

        for line in &lines {
            let _ = out.queue(terminal::Clear(terminal::ClearType::CurrentLine));
            let _ = out.queue(crossterm::style::Print(format!("{}\r\n", line)));
        }

        self.lines_printed = lines.len() as u16;
        let _ = out.flush();
    }
}
