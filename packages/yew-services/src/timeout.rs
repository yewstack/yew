//! This module contains Yew's implementation of a service to
//! send messages when a timeout has elapsed.

use super::Task;
use gloo::timers::callback::Timeout;
use std::convert::TryInto;
use std::fmt;
use std::time::Duration;
use yew::callback::Callback;

/// A handle to cancel a timeout task.
#[must_use = "the timeout will be cleared when the task is dropped"]
pub struct TimeoutTask(Option<Timeout>);

impl fmt::Debug for TimeoutTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TimeoutTask")
    }
}

/// An service to set a timeout.
#[derive(Default, Debug)]
pub struct TimeoutService {}

impl TimeoutService {
    /// Sets timeout which sends messages from a `converter` after `duration`.
    ///
    /// # Panics
    ///
    /// Panics if `duration` in milliseconds exceeds `u32::MAX` (around 50 days).
    pub fn spawn(duration: Duration, callback: Callback<()>) -> TimeoutTask {
        let callback = move || {
            callback.emit(());
        };
        let ms: u32 = duration
            .as_millis()
            .try_into()
            .expect("duration doesn't fit in u32");
        let handle = Timeout::new(ms, callback);
        TimeoutTask(Some(handle))
    }
}

impl Task for TimeoutTask {
    fn is_active(&self) -> bool {
        self.0.is_some()
    }
}
