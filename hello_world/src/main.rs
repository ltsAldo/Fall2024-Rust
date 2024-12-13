mod config;
mod utils;
mod website;
mod worker;

use std::sync::{mpsc::channel, Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use config::Config;
use website::WebsiteStatus;
use worker::worker;
use utils::collect_results;

/// Helper function to format `SystemTime` into a readable UTC-like string
fn format_timestamp_to_utc(timestamp: SystemTime) -> String {
    match timestamp.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let seconds = duration.as_secs();
            let nanoseconds = duration.subsec_nanos();
            let millis = nanoseconds / 1_000_000;
            format!("{:?}.{:03} UTC", seconds, millis)
        }
        Err(_) => "Invalid timestamp".to_string(),
    }
}

fn main() {
    // Initialize configuration
    let config = Config::default();
    println!("Using configuration: {:?}", config);

    // Define URLs to check
    // Add up to 50 URLs here for large-scale testing
    let urls = vec![
        "http://facebook.com".to_string(),
        "http://google.com".to_string(),
        "http://youtube.com".to_string(),
        // Add more URLs as needed 
    ];

    // Create channels for sending URLs and receiving results
    let (tx, rx) = channel::<String>();
    let (result_tx, result_rx) = channel::<WebsiteStatus>();

    // Wrap the Receiver in an Arc<Mutex> for thread-safe shared access
    let rx = Arc::new(Mutex::new(rx));

    // Spawn worker threads
    let mut handles = Vec::new();
    for _ in 0..config.num_threads {
        let rx_clone = Arc::clone(&rx);
        let tx_clone = result_tx.clone();
        let max_retries = config.max_retries; // Pass max_retries from config
        let timeout = config.timeout;

        let handle = thread::spawn(move || worker(rx_clone, tx_clone, timeout, max_retries)); // Pass max_retries
        handles.push(handle);
    }

    // Drop result_tx to signal no more results will be sent
    drop(result_tx);

    // Send URLs to the workers
    for url in &urls {
        println!("Sending URL: {}", url);
        tx.send(url.to_string()).expect("Failed to send URL");
    }

    // Close the sender channel
    drop(tx);
    println!("All URLs sent. Waiting for workers to finish...");

    // Wait for all worker threads to finish
    for handle in handles {
        handle.join().expect("Worker thread panicked");
    }
    println!("All workers finished.");

    // Collect results from the result channel
    println!("Collecting results...");
    let results = collect_results(result_rx);

    // Display the results in the requested struct format
    println!("\n--- Final Results ---");
    for result in results {
        let status = match result.status {
            Ok(code) => format!("Ok({})", code),
            Err(err) => format!("Err(\"{}\")", err),
        };

        println!(
            "WebsiteStatus {{\n    url: \"{}\",\n    status: {},\n    response_time: {:?},\n    timestamp: {},\n}}",
            result.url,
            status,
            result.response_time,
            format_timestamp_to_utc(result.timestamp),
        );
    }

    println!("\nWebsite status check complete.");
}
