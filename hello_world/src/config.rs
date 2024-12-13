#[derive(Debug)]
pub struct Config {
    pub num_threads: usize,
    pub timeout: std::time::Duration,
    pub max_retries: usize, // Add this field
}

impl Config {
    pub fn default() -> Self {
        Self {
            num_threads: 5, // Adjust as needed
            timeout: std::time::Duration::from_secs(5),
            max_retries: 3, // Default to 3 retries
        }
    }
}
