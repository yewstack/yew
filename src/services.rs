use std::time::Duration;
use stdweb::Value;
use html::Context;

pub trait Handle {
    fn cancel(&mut self);
}

pub struct TimeoutHandle {
    timeout_id: Option<Value>,
}

pub trait Timeout<MSG> {
    fn timeout<F>(&mut self, duration: Duration, converter: F) -> TimeoutHandle
    where
        F: Fn() -> MSG + 'static;
}

impl<MSG: 'static> Timeout<MSG> for Context<MSG> {
    fn timeout<F>(&mut self, duration: Duration, converter: F) -> TimeoutHandle
    where
        F: Fn() -> MSG + 'static,
    {
        let mut tx = self.sender();
        let callback = move || {
            let msg = converter();
            tx.send(msg);
        };
        let mut ms = duration.subsec_nanos() / 1_000_000;
        ms += duration.as_secs() as u32 * 1000;
        let id = js! {
            var callback = @{callback};
            let action = function() {
                callback();
                callback.drop();
            };
            let duration = @{ms};
            return setTimeout(callback, duration);
        };
        TimeoutHandle {
            timeout_id: Some(id),
        }
    }
}

impl Handle for TimeoutHandle {
    fn cancel(&mut self) {
        let timeout_id = self.timeout_id.take().expect("tried to cancel timeout twice");
        js! {
            var id = @{timeout_id};
            clearTimeout(id);
        }
    }
}
