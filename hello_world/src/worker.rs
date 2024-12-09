use crate::website::WebsiteStatus;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Instant;
use ureq;

pub fn worker(
    id: usize,
    rx: Receiver<String>,
    tx: Sender<WebsiteStatus>,
    timeout: u64,
) {
    for url in rx {
        let start = Instant::now();
        let result = ureq::get(&url)
            .timeout_connect(timeout as usize)
            .call()
            .map(|resp| resp.status())
            .map_err(|err| err.to_string());
        
        let response_time = start.elapsed();
        let status = WebsiteStatus::new(url.clone(), result, response_time);

        tx.send(status).expect("Failed to send WebsiteStatus");
    }
}
