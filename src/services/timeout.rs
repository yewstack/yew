//! This module contains the implementation of a service to
//! send a messages when timeout elapsed.

use super::{to_ms, Task};
use crate::callback::Callback;
#[cfg(feature = "web_sys")]
use gloo::timers::callback::Timeout;
use std::fmt;
use std::time::Duration;
#[cfg(feature = "std_web")]
use stdweb::Value;
#[cfg(feature = "std_web")]
#[allow(unused_imports)]
use stdweb::{_js_impl, js};

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
        #[cfg(feature = "std_web")]
        let handle = js! {
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
        };
        #[cfg(feature = "web_sys")]
        let handle = Timeout::new(ms, callback);
        TimeoutTask(Some(handle))
    }
}

impl Task for TimeoutTask {
    fn is_active(&self) -> bool {
        self.0.is_some()
    }
    fn cancel(&mut self) {
        let handle = self.0.take().expect("tried to cancel timeout twice");
        #[cfg(feature = "std_web")]
        js! { @(no_return)
            var handle = @{handle};
            clearTimeout(handle.timeout_id);
            handle.callback.drop();
        }
        #[cfg(feature = "web_sys")]
        handle.cancel();
    }
}

impl Drop for TimeoutTask {
    fn drop(&mut self) {
        if self.is_active() {
            self.cancel();
        }
    }
}
