use crate::metrics::SpeedProgress;
use crate::renderer::chart;
use crate::renderer::{CHART_HEIGHT, CHART_WIDTH};
use crossterm::style::Stylize;

pub fn render_phase_header(label: &str, prog: &Option<SpeedProgress>, active: bool) -> String {
    match prog {
        Some(p) => format!(
            "  {}  {:>8.1} Mbps  ▸  avg {:>7.1}  peak {:>7.1}  {:.1}s",
            label.bold(),
            p.current_mbps,
            p.avg_mbps,
            p.peak_mbps,
            p.elapsed_secs,
        ),
        None => {
            let status = if active {
                "◷ measuring…"
            } else {
                "pending"
            };
            format!("  {}  {}", label.bold(), status)
        }
    }
}

pub fn render_phase_body(lines: &mut Vec<String>, prog: &Option<SpeedProgress>) {
    match prog {
        Some(p) if !p.history.is_empty() => {
            // Sparkline bar
            let spark = chart::sparkline(&p.history, CHART_WIDTH);
            lines.push(format!("  {}", spark));
            lines.push(String::new());

            // Line chart
            let chart_lines = chart::line_chart(&p.history, CHART_WIDTH, CHART_HEIGHT);
            for cl in &chart_lines {
                lines.push(format!("  {}", cl));
            }
        }
        _ => {
            // Empty placeholder so the frame height stays constant.
            for _ in 0..(CHART_HEIGHT + 3) {
                lines.push(String::new());
            }
        }
    }
}
