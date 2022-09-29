use std::time::Duration;

use chrono::{DateTime, Local};
use futures::channel::mpsc;
use futures::Stream;
use wasm_bindgen_futures::spawn_local;
use yew::platform::time::sleep;

/// Simple Clock
pub struct Clock;

const ONE_SEC: Duration = Duration::from_secs(1);

impl Clock {
    /// Create a new clock.
    pub fn new() -> Self {
        Clock
    }

    /// Returns a stream of time updates.
    ///
    /// Note: this isn't the most efficient way of creating a stream of time updates. Its main
    /// purpose is to show how to combine async code with yew components.
    pub fn stream_time(&self) -> impl Stream<Item = DateTime<Local>> {
        // Create a sender and receiver pair.
        let (tx, rx) = mpsc::unbounded();

        // Spawn a background task that will send the current time to the receiver every second.
        spawn_local(async move {
            // Wait a bit before starting the stream, so the user can observe the "initialized"
            // state.
            sleep(Duration::from_secs(2)).await;

            loop {
                let now = Local::now();
                tx.unbounded_send(now).expect("Failed to send time");
                sleep(ONE_SEC).await;
            }
        });

        // Return the receiver. Note that this method returns immediately, and that any work is done
        // in the background.
        rx
    }
}

/// Demonstration code to show how to use async code in a yew component.
pub async fn initialize_atomic_clocks() {
    // aligning with atomic clocks :-)
    sleep(ONE_SEC).await;
}
