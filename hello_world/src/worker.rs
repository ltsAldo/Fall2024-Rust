use crate::website::WebsiteStatus;
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Instant, Duration};
use ureq;

pub fn worker(
    rx: Arc<Mutex<mpsc::Receiver<String>>>,
    tx: mpsc::Sender<WebsiteStatus>,
    timeout: Duration,
) {
    loop {
        println!("Worker waiting for URL...");
        let url = {
            let lock = rx.lock().expect("Failed to lock receiver");
            let url = lock.recv().ok(); // recv() returns None if the channel is closed
            drop(lock); // Explicitly release the lock
            url
        };

        if let Some(url) = url {
            println!("Worker processing URL: {}", url);
            let start = Instant::now();
            let result = ureq::get(&url)
                .timeout(timeout)
                .call()
                .map(|resp| resp.status())
                .map_err(|err| err.to_string());

            let response_time = start.elapsed();
            let status = WebsiteStatus::new(url, result, response_time);

            println!("Worker finished processing URL: {:?}", status);
            tx.send(status).expect("Failed to send WebsiteStatus");
        } else {
            println!("Worker exiting: no more URLs.");
            break; // Exit loop when channel is closed
        }
    }
}
