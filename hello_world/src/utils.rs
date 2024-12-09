use std::sync::mpsc::Receiver;

pub fn collect_results<T>(rx: Receiver<T>) -> Vec<T> {
    let mut results = Vec::new();
    for result in rx {
        results.push(result);
    }
    results
}
