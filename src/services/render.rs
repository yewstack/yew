//! This module contains the implementation of a service to
//! request frame rendering

use crate::callback::Callback;
use crate::services::Task;
use std::fmt;
#[cfg(feature = "std_web")]
#[allow(unused_imports)]
use stdweb::{_js_impl, js};
#[cfg(feature = "std_web")]
use stdweb::{unstable::TryInto, Value};
#[cfg(feature = "web_sys")]
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

/// A handle to cancel a render task.
#[must_use]
pub struct RenderTask(
    #[cfg(feature = "std_web")] Option<Value>,
    #[cfg(feature = "web_sys")] Option<RenderTaskInner>,
);

#[cfg(feature = "web_sys")]
struct RenderTaskInner {
    render_id: i32,
    #[allow(dead_code)]
    callback: Closure<dyn Fn(JsValue)>,
}

impl fmt::Debug for RenderTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RenderTask")
    }
}

/// A service to request animation frames.
#[derive(Default, Debug)]
pub struct RenderService {}

impl RenderService {
    /// Create a new service instance
    pub fn new() -> Self {
        Self {}
    }

    /// Request animation frame. Callback will be notified when frame should be rendered.
    pub fn request_animation_frame(&mut self, callback: Callback<f64>) -> RenderTask {
        let callback = move |#[cfg(feature = "std_web")] v,
                             #[cfg(feature = "web_sys")] v: JsValue| {
            #[cfg(feature = "std_web")]
            let time: f64 = match v {
                Value::Number(n) => n.try_into().unwrap(),
                _ => 0.0,
            };
            #[cfg(feature = "web_sys")]
            let time = v.as_f64().unwrap_or(0.);
            callback.emit(time);
        };
        #[cfg(feature = "std_web")]
        let handle = js! {
            var callback = @{callback};
            var action = function(time) {
                callback(time);
                callback.drop();
            };
            return {
                render_id: requestAnimationFrame(action),
                callback: callback,
            };
        };
        #[cfg(feature = "web_sys")]
        let handle = {
            let callback = Closure::wrap(Box::new(callback) as Box<dyn Fn(JsValue)>);
            let render_id = web_sys::window()
                .unwrap()
                .request_animation_frame(callback.as_ref().unchecked_ref())
                .unwrap();
            RenderTaskInner {
                render_id,
                callback,
            }
        };
        RenderTask(Some(handle))
    }
}

impl Task for RenderTask {
    fn is_active(&self) -> bool {
        self.0.is_some()
    }
    fn cancel(&mut self) {
        let handle = self.0.take().expect("tried to cancel render twice");
        #[cfg(feature = "std_web")]
        js! { @(no_return)
            var handle = @{handle};
            cancelAnimationFrame(handle.render_id);
            handle.callback.drop();
        }
        #[cfg(feature = "web_sys")]
        web_sys::window()
            .unwrap()
            .cancel_animation_frame(handle.render_id)
            .unwrap();
    }
}

impl Drop for RenderTask {
    fn drop(&mut self) {
        if self.is_active() {
            self.cancel();
        }
    }
}
