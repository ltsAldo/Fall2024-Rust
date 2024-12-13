use std::sync::mpsc::Receiver;

/// Collects all results from the receiver into a vector.
pub fn collect_results<T>(rx: Receiver<T>) -> Vec<T> {
    let mut results = Vec::new();
    while let Ok(result) = rx.recv() {
        results.push(result);
    }
    results
}
