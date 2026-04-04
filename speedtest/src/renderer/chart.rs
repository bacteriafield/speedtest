const SPARK_CHARS: &[char] = &['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

pub fn sparkline(data: &[f64], width: usize) -> String {
    if data.is_empty() {
        return " ".repeat(width);
    }

    let window: Vec<f64> =
        if data.len() > width { data[data.len() - width..].to_vec() } else { data.to_vec() };

    let max = window.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min = window.iter().cloned().fold(f64::INFINITY, f64::min);
    let range = (max - min).max(1e-9);

    window
        .iter()
        .map(|&v| {
            let idx = (((v - min) / range) * (SPARK_CHARS.len() - 1) as f64).round() as usize;
            SPARK_CHARS[idx.min(SPARK_CHARS.len() - 1)]
        })
        .collect()
}

/*******************************************************************************/
/// Renders a line chart as a vector of strings (each = one terminal row).      
///
/// # Layout
/// ```
/// 500 ┤        ╭──╮
/// 400 ┤──╮  ╭──╯  ╰──
/// 300 ┤  ╰──╯
///     └────────────────
/// ```
/*******************************************************************************/
pub fn line_chart(data: &[f64], width: usize, height: usize) -> Vec<String> {
    if data.is_empty() || width == 0 || height == 0 {
        return vec![" ".repeat(width + 6); height + 1];
    }

    let data_width = width.saturating_sub(1); // leave space for Y axis

    // Use only the last `data_width` samples.
    let window: Vec<f64> = {
        let n = data.len();
        if n > data_width { data[n - data_width..].to_vec() } else { data.to_vec() }
    };

    let max = window.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min = window.iter().cloned().fold(f64::INFINITY, f64::min);
    let range = (max - min).max(1.0);

    // Map value → row (0 = top = high, height-1 = bottom = low).
    let to_row = |v: f64| -> i32 {
        let ratio = (v - min) / range;
        ((1.0 - ratio) * (height as f64 - 1.0)).round() as i32
    };

    let mut grid: Vec<Vec<char>> = vec![vec![' '; data_width]; height];

    for (x, &v) in window.iter().enumerate() {
        let curr = to_row(v);
        let prev = if x == 0 { curr } else { to_row(window[x - 1]) };

        if curr == prev {
            grid[curr as usize][x] = '─';
        } else {
            let (upper, lower) = if curr < prev { (curr, prev) } else { (prev, curr) };

            // Top character
            grid[upper as usize][x] = if curr < prev { '╭' } else { '╮' };

            // Fill middle
            for r in (upper + 1)..lower {
                grid[r as usize][x] = '│';
            }

            // Bottom character
            grid[lower as usize][x] = if curr < prev { '╯' } else { '╰' };
        }
    }

    // Build Y-axis labels + axis line.
    let tick_step = range / (height as f64 - 1.0);
    let mut rows: Vec<String> = Vec::with_capacity(height + 1);

    for r in 0..height {
        let value = max - r as f64 * tick_step;
        let label = if value >= 1000.0 {
            format!("{:>4.0}G", value / 1000.0)
        } else {
            format!("{:>4.0} ", value)
        };
        let line: String = grid[r].iter().collect();
        rows.push(format!("{} ┤ {}", label, line));
    }

    // Bottom axis
    rows.push(format!("     └─{}", "─".repeat(data_width)));

    rows
}
