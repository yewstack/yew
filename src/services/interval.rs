use std::time::Duration;
use stdweb::Value;
use html::Context;
use super::{Task, to_ms};

pub struct IntervalHandle {
    interval_id: Option<Value>,
}

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
        let id = js! {
            var callback = @{callback};
            let action = function() {
                callback();
            };
            let delay = @{ms};
            return setInterval(action, delay);
        };
        IntervalHandle {
            interval_id: Some(id),
        }
    }
}

impl Task for IntervalHandle {
    fn cancel(&mut self) {
        let interval_id = self.interval_id.take().expect("tried to cancel interval twice");
        js! {
            // TODO Drop the callback to prevent memory leak
            var id = @{interval_id};
            clearInterval(id);
        }
    }
}
