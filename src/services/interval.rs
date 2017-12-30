//! This module contains the implementation of a service for
//! periodic sending messages to a loop.

use std::time::Duration;
use stdweb::Value;
use html::Context;
use super::{Task, to_ms};

/// A handle which helps to cancel interval. Uses
/// [clearInterval](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/clearInterval).
pub struct IntervalHandle(Option<Value>);

/// An abstract service to send messages on every elapsed interval.
pub trait IntervalService<MSG> {
    /// Sets interval which will call send a messages returned by a converter
    /// on every intarval expiration.
    fn interval<F>(&mut self, duration: Duration, converter: F) -> IntervalHandle
    where
        F: Fn() -> MSG + 'static;
}

impl<MSG: 'static> IntervalService<MSG> for Context<MSG> {
    fn interval<F>(&mut self, duration: Duration, converter: F) -> IntervalHandle
    where
        F: Fn() -> MSG + 'static,
    {
        let mut tx = self.sender();
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
        js! {
            var handle = @{handle};
            clearInterval(handle.interval_id);
            handle.callback.drop();
        }
    }
}
