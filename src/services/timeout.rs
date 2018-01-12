//! This module contains the implementation of a service to
//! send a messages when timeout elapsed.

use std::time::Duration;
use stdweb::Value;
use html::AppSender;
use super::{Task, to_ms};

/// A handle to cancel a timeout task.
pub struct TimeoutHandle(Option<Value>);

/// An service to set a timeout.
pub struct TimeoutService<MSG> {
    sender: AppSender<MSG>,
}

impl<MSG: 'static> TimeoutService<MSG> {
    /// Creates a new service instance connected to `App` by provided `sender`.
    pub fn new(sender: AppSender<MSG>) -> Self {
        Self { sender }
    }

    /// Sets timeout which send a messages from a `converter` after `duration`.
    pub fn spawn<F>(&mut self, duration: Duration, converter: F) -> TimeoutHandle
    where
        F: Fn() -> MSG + 'static,
    {
        let mut tx = self.sender.clone();
        let callback = move || {
            let msg = converter();
            tx.send(msg);
        };
        let ms = to_ms(duration);
        let handle = js! {
            var callback = @{callback};
            let action = function() {
                callback();
                callback.drop();
            };
            let delay = @{ms};
            return {
                timeout_id: setTimeout(action, delay),
                callback,
            };
        };
        TimeoutHandle(Some(handle))
    }
}

impl Task for TimeoutHandle {
    fn cancel(&mut self) {
        let handle = self.0.take().expect("tried to cancel timeout twice");
        js! { @(no_return)
            var handle = @{handle};
            clearTimeout(handle.timeout_id);
            handle.callback.drop();
        }
    }
}
