pub struct Config {
    pub num_threads: usize,
    pub timeout: std::time::Duration,
}

impl Config {
    pub fn default() -> Self {
        Self {
            num_threads: 4, // Number of worker threads
            timeout: std::time::Duration::from_secs(5), // Timeout duration
        }
    }
}
