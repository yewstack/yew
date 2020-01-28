//! This module contains the implementation of a service for
//! periodic sending messages to a loop.

use super::{to_ms, Task};
use crate::callback::Callback;
use std::fmt;
use std::time::Duration;
use stdweb::Value;
#[allow(unused_imports)]
use stdweb::{_js_impl, js};

/// A handle which helps to cancel interval. Uses
/// [clearInterval](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/clearInterval).
#[must_use]
pub struct IntervalTask(Option<Value>);

impl fmt::Debug for IntervalTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("IntervalTask")
    }
}

/// A service to send messages on every elapsed interval.
#[derive(Default, Debug)]
pub struct IntervalService {}

impl IntervalService {
    /// Creates a new service instance connected to `App` by provided `sender`.
    pub fn new() -> Self {
        Self {}
    }

    /// Sets interval which will call send a messages returned by a converter
    /// on every intarval expiration.
    pub fn spawn(&mut self, duration: Duration, callback: Callback<()>) -> IntervalTask {
        let callback = move || {
            callback.emit(());
        };
        let ms = to_ms(duration);
        let handle = js! {
            var callback = @{callback};
            var action = function() {
                callback();
            };
            var delay = @{ms};
            return {
                interval_id: setInterval(action, delay),
                callback: callback,
            };
        };
        IntervalTask(Some(handle))
    }
}

impl Task for IntervalTask {
    fn is_active(&self) -> bool {
        self.0.is_some()
    }
}

impl Drop for IntervalTask {
    fn drop(&mut self) {
        if self.is_active() {
            let handle = self.0.take().unwrap();
            js! { @(no_return)
                var handle = @{handle};
                clearInterval(handle.interval_id);
                handle.callback.drop();
            }
        }
    }
}
