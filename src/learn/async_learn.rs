use tokio::time::{Duration, sleep};

/// Simulate an async IO call that takes `delay_ms` milliseconds to complete.
async fn mock_request(label: &'static str, delay_ms: u64) -> String {
    sleep(Duration::from_millis(delay_ms)).await;
    format!(
        "task `{label}` finished on thread {:?}",
        std::thread::current().id()
    )
}

/// Run two async tasks concurrently and return their output strings.
pub async fn run_async_example() -> Vec<String> {
    let fast = tokio::spawn(async { mock_request("fast", 200).await });
    let slow = tokio::spawn(async { mock_request("slow", 600).await });

    let mut results = Vec::new();
    results.push(fast.await.expect("fast task panicked"));
    results.push(slow.await.expect("slow task panicked"));
    results
}

/// Helper to run the async example from sync code (e.g. tests or binaries).
pub fn init() {
    tokio::runtime::Runtime::new()
        .expect("create tokio runtime")
        .block_on(run_async_example())
        .iter()
        .for_each(|s| println!("{s}"));
}
