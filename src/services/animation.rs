//! This module contains the implementation of a service for
//! requesting synchronized animation frames.
//! [Timing control for script-based animations](https://www.w3.org/TR/animation-timing/).

use stdweb::Value;

use html::Callback;
use super::Task;


/// A handle which helps to cancel request. Uses
/// [cancelAnimationFrame](https://developer.mozilla.org/en-US/docs/Web/API/Window/cancelAnimationFrame).
pub struct AnimationHandle(Option<Value>);

/// A services to send messages when frame is ready to render.
pub struct AnimationService {
}

impl AnimationService {
    /// Creates a new services instance connected to `App` by provided `sender`.
    pub fn new() -> Self {
        Self { }
    }

    /// Starts animation.
    pub fn spawn(&mut self, callback: Callback<f64>) -> AnimationHandle
    {
        let callback = move |tm: f64| {
            callback.emit(tm);
        };
        let handle = js! {
            var callback = @{callback};
            var action = action_;
            var handle = {
                request_id: requestAnimationFrame(action),
                callback: callback,
            };
            return handle;

            function action_(tm) {
                handle.request_id = requestAnimationFrame(action);
                callback(tm);
            };
        };
        AnimationHandle(Some(handle))
    }
}

impl Task for AnimationHandle {
    fn is_active(&self) -> bool {
        self.0.is_some()
    }

    fn cancel(&mut self) {
        let handle = self.0.take().expect("tried to cancel an animation request twice");
        js! { @(no_return)
            var handle = @{handle};
            cancelAnimationFrame(handle.request_id);
            handle.callback.drop();
        }
    }
}

impl Drop for AnimationHandle {
    fn drop(&mut self) {
        if self.is_active() {
            self.cancel();
        }
    }
}
