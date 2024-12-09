use std::time::Duration;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
}

impl WebsiteStatus {
    pub fn new(url: String, status: Result<u16, String>, response_time: Duration) -> Self {
        Self {
            url,
            status,
            response_time,
            timestamp: Utc::now(),
        }
    }
}
