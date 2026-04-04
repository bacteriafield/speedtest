#[allow(dead_code)]
#[allow(unused)]
pub struct CircularBuffer {
    data: Vec<f64>,
    capacity: usize,
    head: usize,
    len: usize,
}

impl CircularBuffer {
    pub fn new(capacity: usize) -> Self {
        Self { data: vec![0.0; capacity], capacity, head: 0, len: 0 }
    }

    pub fn push(&mut self, value: f64) {
        self.data[self.head] = value;
        self.head = (self.head + 1) % self.capacity;
        if self.len < self.capacity {
            self.len += 1;
        }
    }

    /// Returns elements in insertion order (oldest first).
    pub fn as_ordered(&self) -> Vec<f64> {
        if self.len == 0 {
            return vec![];
        }
        let start = if self.len < self.capacity { 0 } else { self.head };
        (0..self.len).map(|i| self.data[(start + i) % self.capacity]).collect()
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[derive(Debug, Clone, Default)]
pub struct Stats {
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub jitter: f64,
    pub samples: usize,
}

impl Stats {
    pub fn from_samples(samples: &[f64]) -> Self {
        if samples.is_empty() {
            return Self::default();
        }

        let min = samples.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = samples.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let avg = samples.iter().sum::<f64>() / samples.len() as f64;

        let jitter = if samples.len() > 1 {
            let diffs: Vec<f64> = samples.windows(2).map(|w| (w[1] - w[0]).abs()).collect();
            diffs.iter().sum::<f64>() / diffs.len() as f64
        } else {
            0.0
        };

        Self { min, max, avg, jitter, samples: samples.len() }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpeedProgress {
    pub current_mbps: f64,
    pub avg_mbps: f64,
    pub peak_mbps: f64,
    pub elapsed_secs: f64,
    pub history: Vec<f64>, // for chart
}

#[derive(Debug, Clone, Default)]
pub struct PingResult {
    pub avg_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub jitter_ms: f64,
}

#[derive(Debug, Clone, Default)]
pub struct TestResult {
    pub server_name: String,
    pub server_id: String,
    pub ping: PingResult,
    pub download_mbps: f64,
    pub upload_mbps: f64,
    pub timestamp: String,
}
