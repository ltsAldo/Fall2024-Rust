mod config;
mod website;
mod worker;
mod utils;

use config::Config;
use website::WebsiteStatus;
use worker::worker;
use utils::collect_results;
use std::sync::{mpsc::channel, Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

/// Helper function to format `SystemTime` into a human-readable string.
fn format_timestamp(timestamp: SystemTime) -> String {
    match timestamp.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let secs = duration.as_secs();
            let nanos = duration.subsec_nanos();
            format!("{:?}.{:09} UTC", secs, nanos)
        }
        Err(_) => "Invalid timestamp".to_string(),
    }
}

fn main() {
    // Load configuration
    let config = Config::default();

    // Define URLs to check
    let urls = vec![
        "http://facebook.com".to_string(),
        "http://google.com".to_string(),
        "http://youtube.com".to_string(),
    ];

    // Create channels for sending URLs and receiving results
    let (tx, rx) = channel::<String>();
    let (result_tx, result_rx) = channel::<WebsiteStatus>();

    // Wrap the Receiver in an Arc<Mutex> for thread-safe shared access
    let rx = Arc::new(Mutex::new(rx));

    // Spawn worker threads
    for _ in 0..config.num_threads {
        let rx_clone = Arc::clone(&rx); // Clone the Arc to share it among threads
        let tx_clone = result_tx.clone();
        thread::spawn(move || worker(rx_clone, tx_clone, config.timeout));
    }

    // Send URLs to the workers
    for url in &urls {
        tx.send(url.to_string()).expect("Failed to send URL");
    }

    // Close the sender channel to signal no more URLs will be sent
    drop(tx);

    // Collect results from the result channel
    let results = collect_results(result_rx);

    // Display the results in summary format
    println!("\n--- Final Results ---");
    for result in &results {
        let status = match &result.status {
            Ok(code) => format!("Ok({})", code),
            Err(err) => format!("Err(\"{}\")", err),
        };

        println!(
            "WebsiteStatus {{\n    url: \"{}\",\n    status: {},\n    response_time: {:?},\n    timestamp: {},\n}}",
            result.url,
            status,
            result.response_time,
            format_timestamp(result.timestamp),
        );
    }

    println!("\nWebsite status check complete.");
}
