use std::time::Duration;

use chrono::Local;
use gloo_timers::future::sleep;
use tokio_stream::wrappers::ReceiverStream;
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
    pub fn stream_time(&self) -> ReceiverStream<String> {
        // Create a sender and receiver pair.
        let (tx, rx) = tokio::sync::mpsc::channel(10);

        // Spawn a background task that will send the current time to the receiver every second.
        spawn_local(async move {
            // Wait a bit before starting the stream, so the user can observe the "initialized"
            // state.
            sleep(Duration::from_secs(2)).await;

            loop {
                let formatted_time = Self::nice_now();
                tx.send(formatted_time).await.expect("Failed to send time");
                sleep(ONE_SEC).await;
            }
        });

        // Return the receiver. Note that this method returns immediately, and that any work is done
        // in the background.
        ReceiverStream::new(rx)
    }

    fn nice_now() -> String {
        let current_time = Local::now();
        current_time.to_rfc2822()
    }
}

/// Demonstration code to show how to use async code in a yew component.
pub async fn initialized_atomic_clocks() -> String {
    // aligning with atomic clocks :-)
    sleep(ONE_SEC).await;
    "Initialized".to_string()
}
