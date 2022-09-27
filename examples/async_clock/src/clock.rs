use chrono::Local;
use gloo_timers::future::sleep;
use std::time::Duration;
use tokio::sync::mpsc::Receiver;
use wasm_bindgen_futures::spawn_local;

/// Simple Clock
pub struct Clock;

const ONE_SEC: Duration = Duration::from_secs(1);

impl Clock {
    /// Create a new clock.
    pub fn new() -> Self {
        Clock
    }

    /// Returns a stream of time updates.
    pub fn stream_time(&self) -> Receiver<String> {
        // Create a sender and receiver pair.
        let (tx, rx) = tokio::sync::mpsc::channel(10);

        // Spawn a background task that will send the current time to the receiver every second.
        spawn_local(async move {
            loop {
                let current_time = Local::now();
                let formatted_time = current_time.to_rfc2822();
                tx.send(formatted_time).await.expect("Failed to send time");
                sleep(ONE_SEC).await;
            }
        });

        // Return the receiver. Note that this method returns immediately, and that any work is done in the background.
        rx
    }
}
