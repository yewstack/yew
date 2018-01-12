//! This module contains the implementation of a service for
//! periodic sending messages to a loop.

use std::time::Duration;
use stdweb::Value;
use html::AppSender;
use super::{Task, to_ms};

/// A handle which helps to cancel interval. Uses
/// [clearInterval](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/clearInterval).
pub struct IntervalHandle(Option<Value>);

/// A service to send messages on every elapsed interval.
pub struct IntervalService<MSG> {
    sender: AppSender<MSG>,
}

impl<MSG: 'static> IntervalService<MSG> {
    /// Creates a new service instance connected to `App` by provided `sender`.
    pub fn new(sender: AppSender<MSG>) -> Self {
        Self { sender }
    }

    /// Sets interval which will call send a messages returned by a converter
    /// on every intarval expiration.
    pub fn spawn<F>(&mut self, duration: Duration, converter: F) -> IntervalHandle
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
        js! { @(no_return)
            var handle = @{handle};
            clearInterval(handle.interval_id);
            handle.callback.drop();
        }
    }
}
