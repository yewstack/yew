//! This module contains the implementation of a service for
//! periodic sending messages to a loop.

use super::{to_ms, Task};
use crate::callback::Callback;
#[cfg(feature = "web_sys")]
use gloo::timers::callback::Interval;
use std::fmt;
use std::time::Duration;
#[cfg(feature = "std_web")]
use stdweb::Value;
#[cfg(feature = "std_web")]
#[allow(unused_imports)]
use stdweb::{_js_impl, js};

/// A handle which helps to cancel interval. Uses
/// [clearInterval](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/clearInterval).
#[must_use]
pub struct IntervalTask(
    #[cfg(feature = "std_web")] Option<Value>,
    #[cfg(feature = "web_sys")] Option<Interval>,
);

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
        #[cfg(feature = "std_web")]
        {
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
        #[cfg(feature = "web_sys")]
        IntervalTask(Some(Interval::new(ms, callback)))
    }
}

impl Task for IntervalTask {
    fn is_active(&self) -> bool {
        self.0.is_some()
    }
    fn cancel(&mut self) {
        #[cfg_attr(feature = "web_sys", allow(unused_variables))]
        let handle = self.0.take().expect("tried to cancel interval twice");
        #[cfg(feature = "std_web")]
        js! { @(no_return)
            var handle = @{handle};
            clearInterval(handle.interval_id);
            handle.callback.drop();
        }
    }
}

impl Drop for IntervalTask {
    fn drop(&mut self) {
        if self.is_active() {
            self.cancel();
        }
    }
}
