use std::time::Duration;
use stdweb::Value;
use html::Context;
use super::{Task, to_ms};

pub struct IntervalHandle(Option<Value>);

pub trait IntervalService<MSG> {
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
