//! This module contains the implementation of a service for
//! periodic sending messages to a loop.

use std::time::Duration;
use stdweb::Value;
use html::Callback;
use super::{Task, to_ms};

/// A handle which helps to cancel interval. Uses
/// [clearInterval](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/clearInterval).
pub struct IntervalHandle(Option<Value>);

/// A service to send messages on every elapsed interval.
pub struct IntervalService {
}

impl IntervalService {
    /// Creates a new service instance connected to `App` by provided `sender`.
    pub fn new() -> Self {
        Self { }
    }

    /// Sets interval which will call send a messages returned by a converter
    /// on every intarval expiration.
    pub fn spawn(&mut self, duration: Duration, callback: Callback<()>) -> IntervalHandle
    {
        let callback = move || {
            callback.emit(());
        };
        let ms = to_ms(duration);
        let handle = js! {
            var callback = @{callback};
            let action = function() {
                callback();
            };
            let delay = @{ms};
            return {
                interval_id: setInterval(action, delay),
                callback,
            };
        };
        IntervalHandle(Some(handle))
    }
}

impl Task for IntervalHandle {
    fn cancel(&mut self) {
        let handle = self.0.take().expect("tried to cancel interval twice");
        js! {
            var handle = @{handle};
            clearInterval(handle.interval_id);
            handle.callback.drop();
        }
    }
}
