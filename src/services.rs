use std::time::Duration;
use stdweb::Value;
use html::Context;

pub trait Task {
    fn cancel(&mut self);
}

fn to_ms(duration: Duration) -> u32 {
    let ms = duration.subsec_nanos() / 1_000_000;
    ms + duration.as_secs() as u32 * 1000
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
        let ms = to_ms(duration);
        let id = js! {
            var callback = @{callback};
            let action = function() {
                callback();
                callback.drop();
            };
            let delay = @{ms};
            return setTimeout(callback, delay);
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

pub struct IntervalHandle {
    interval_id: Option<Value>,
}

pub trait Interval<MSG> {
    fn interval<F>(&mut self, duration: Duration, converter: F) -> IntervalHandle
    where
        F: Fn() -> MSG + 'static;
}

impl<MSG: 'static> Interval<MSG> for Context<MSG> {
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
                callback.drop();
            };
            let delay = @{ms};
            return setInterval(callback, delay);
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
