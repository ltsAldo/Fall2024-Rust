mod config;
mod website;
mod worker;
mod utils;

use config::Config;
use website::WebsiteStatus;
use worker::worker;
use utils::collect_results;
use std::sync::mpsc::{channel};
use std::thread;

fn main() {
    let config = Config::default();
    let urls = vec![
        "http://example.com",
        "http://google.com",
        "http://nonexistent.url",
    ];

    let (tx, rx) = channel();
    let (result_tx, result_rx) = channel();

    // Spawn worker threads
    for id in 0..config.num_threads {
        let rx = rx.clone();
        let tx = result_tx.clone();
        thread::spawn(move || worker(id, rx, tx, config.timeout));
    }

    // Send URLs to workers
    for url in &urls {
        tx.send(url.to_string()).expect("Failed to send URL");
    }
    drop(tx); // Close channel to signal workers

    // Collect and print results
    let results = collect_results(result_rx);
    for result in results {
        println!(
            "{} - Status: {:?}, Response Time: {:?}, Timestamp: {:?}",
            result.url, result.status, result.response_time, result.timestamp
        );
    }
}
