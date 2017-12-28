use std::time::Duration;
use stdweb::Value;
use html::Context;
use super::{Task, to_ms};

pub struct TimeoutHandle(Option<Value>);

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
        js! {
            var handle = @{handle};
            clearTimeout(handle.timeout_id);
            handle.callback.drop();
        }
    }
}
