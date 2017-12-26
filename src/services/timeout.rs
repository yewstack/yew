use std::time::Duration;
use stdweb::Value;
use html::Context;
use super::{Task, to_ms};

pub struct TimeoutHandle {
    timeout_id: Option<Value>,
}

pub trait TimeoutService<MSG> {
    fn timeout<F>(&mut self, duration: Duration, converter: F) -> TimeoutHandle
    where
        F: Fn() -> MSG + 'static;
}

impl<MSG: 'static> TimeoutService<MSG> for Context<MSG> {
    fn timeout<F>(&mut self, duration: Duration, converter: F) -> TimeoutHandle
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
                callback.drop();
            };
            let delay = @{ms};
            return setTimeout(action, delay);
        };
        TimeoutHandle {
            timeout_id: Some(id),
        }
    }
}

impl Task for TimeoutHandle {
    fn cancel(&mut self) {
        let timeout_id = self.timeout_id.take().expect("tried to cancel timeout twice");
        js! {
            // TODO Drop the callback to prevent memory leak
            var id = @{timeout_id};
            clearTimeout(id);
        }
    }
}
