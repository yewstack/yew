//! This module contains the implementation of a service for
//! periodic sending messages to a loop.

use super::Task;
use gloo::timers::callback::Interval;
use std::convert::TryInto;
use std::fmt;
use std::time::Duration;
use yew::callback::Callback;

/// A handle which helps to cancel interval. Uses
/// [clearInterval](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/clearInterval).
#[must_use = "the interval is only active until the handle is dropped"]
pub struct IntervalTask(Interval);

impl fmt::Debug for IntervalTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("IntervalTask")
    }
}

/// A service to send messages on every elapsed interval.
#[derive(Default, Debug)]
pub struct IntervalService {}

impl IntervalService {
    /// Sets interval which will call send a messages returned by a converter
    /// on every interval expiration.
    ///
    /// # Panics
    ///
    /// Panics if `duration` in milliseconds exceeds `u32::MAX` (around 50 days).
    pub fn spawn(duration: Duration, callback: Callback<()>) -> IntervalTask {
        let callback = move || {
            callback.emit(());
        };
        let ms: u32 = duration
            .as_millis()
            .try_into()
            .expect("duration doesn't fit in u32");
        let handle = Interval::new(ms, callback);
        IntervalTask(handle)
    }
}

impl Task for IntervalTask {
    fn is_active(&self) -> bool {
        true
    }
}
