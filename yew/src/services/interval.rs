//! This module contains the implementation of a service for
//! periodic sending messages to a loop.

use super::{to_ms, Task};
use crate::callback::Callback;
use cfg_if::cfg_if;
use cfg_match::cfg_match;
use std::fmt;
use std::time::Duration;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::Value;
        #[allow(unused_imports)]
        use stdweb::{_js_impl, js};
    } else if #[cfg(feature = "web_sys")] {
        use gloo::timers::callback::Interval;
    }
}

/// A handle which helps to cancel interval. Uses
/// [clearInterval](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/clearInterval).
#[must_use]
pub struct IntervalTask(
    #[cfg(feature = "std_web")] Value,
    #[cfg(feature = "web_sys")] Interval,
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
        let handle = cfg_match! {
            feature = "std_web" => js! {
                var callback = @{callback};
                var action = function() {
                    callback();
                };
                var delay = @{ms};
                return {
                    interval_id: setInterval(action, delay),
                    callback: callback,
                };
            },
            feature = "web_sys" => Interval::new(ms, callback),
        };
        IntervalTask(handle)
    }
}

impl Task for IntervalTask {
    fn is_active(&self) -> bool {
        true
    }
}

impl Drop for IntervalTask {
    fn drop(&mut self) {
        #[cfg(feature = "std_web")]
        {
            if self.is_active() {
                let handle = &self.0;
                js! { @(no_return)
                    var handle = @{handle};
                    clearInterval(handle.interval_id);
                    handle.callback.drop();
                }
            }
        }
    }
}
