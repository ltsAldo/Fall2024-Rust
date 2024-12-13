use crate::website::WebsiteStatus;
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Instant, Duration};
use ureq;

pub fn worker(
    rx: Arc<Mutex<mpsc::Receiver<String>>>,
    tx: mpsc::Sender<WebsiteStatus>,
    timeout: Duration,
    max_retries: usize, // New parameter for retries
) {
    loop {
        let url = {
            let lock = rx.lock().unwrap();
            lock.recv().ok()
        };

        if let Some(url) = url {
            println!("Worker processing URL: {}", url);

            let mut attempt = 0;
            let mut result;
            let mut response_time;

            loop {
                let start = Instant::now();
                result = ureq::get(&url)
                    .timeout(timeout)
                    .call()
                    .map(|resp| resp.status())
                    .map_err(|err| err.to_string());
                response_time = start.elapsed();

                if result.is_ok() || attempt >= max_retries - 1 {
                    break;
                }

                println!(
                    "Retrying URL: {} (Attempt {}/{})",
                    url, attempt + 1, max_retries
                );
                attempt += 1;
            }

            let status = WebsiteStatus::new(url.clone(), result, response_time);

            // Send the result back to main
            if tx.send(status).is_err() {
                println!("Failed to send result: Receiver dropped.");
                break;
            }
        } else {
            println!("Worker exiting: No more URLs.");
            break;
        }
    }
}
