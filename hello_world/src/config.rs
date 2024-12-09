pub struct Config {
    pub num_threads: usize,
    pub timeout: u64,
    pub max_retries: usize,
}

impl Config {
    pub fn default() -> Self {
        Self {
            num_threads: 10,
            timeout: 5000,
            max_retries: 3,
        }
    }
}
