//! This module contains Yew's implementation of a service to
//! send messages when a timeout has elapsed.

use super::Task;
use crate::callback::Callback;
use cfg_if::cfg_if;
use cfg_match::cfg_match;
use std::convert::TryInto;
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
#[must_use = "the timeout will be cleared when the task is dropped"]
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
