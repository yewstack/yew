//! This module contains the implementation of a service to
//! send a messages when timeout elapsed.

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
        use gloo::timers::callback::Timeout;
    }
}

/// A handle to cancel a timeout task.
#[must_use]
pub struct TimeoutTask(
    #[cfg(feature = "std_web")] Option<Value>,
    #[cfg(feature = "web_sys")] Option<Timeout>,
);

impl fmt::Debug for TimeoutTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TimeoutTask")
    }
}

/// An service to set a timeout.
#[derive(Default, Debug)]
pub struct TimeoutService {}

impl TimeoutService {
    /// Creates a new service instance connected to `App` by provided `sender`.
    pub fn new() -> Self {
        Self {}
    }

    /// Sets timeout which send a messages from a `converter` after `duration`.
    pub fn spawn(&mut self, duration: Duration, callback: Callback<()>) -> TimeoutTask {
        let callback = move || {
            callback.emit(());
        };
        let ms = to_ms(duration);
        let handle = cfg_match! {
            feature = "std_web" => js! {
                var callback = @{callback};
                var action = function() {
                    callback();
                    callback.drop();
                };
                var delay = @{ms};
                return {
                    timeout_id: setTimeout(action, delay),
                    callback: callback,
                };
            },
            feature = "web_sys" => Timeout::new(ms, callback),
        };
        TimeoutTask(Some(handle))
    }
}

impl Task for TimeoutTask {
    fn is_active(&self) -> bool {
        self.0.is_some()
    }
}

impl Drop for TimeoutTask {
    fn drop(&mut self) {
        #[cfg(feature = "std_web")]
        {
            if self.is_active() {
                let handle = &self.0;
                js! { @(no_return)
                    var handle = @{handle};
                    clearTimeout(handle.timeout_id);
                    handle.callback.drop();
                }
            }
        }
    }
}
