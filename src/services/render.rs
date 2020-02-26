//! This module contains the implementation of a service to
//! request frame rendering

use crate::callback::Callback;
use crate::services::Task;
use cfg_if::cfg_if;
use cfg_match::cfg_match;
use std::fmt;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        #[allow(unused_imports)]
        use stdweb::{_js_impl, js};
        use stdweb::unstable::TryInto;
        use stdweb::Value;
    } else if #[cfg(feature = "web_sys")] {
        use crate::utils;
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::{JsCast, JsValue};
    }
}

/// A handle to cancel a render task.
#[must_use]
pub struct RenderTask(
    #[cfg(feature = "std_web")] Value,
    #[cfg(feature = "web_sys")] RenderTaskInner,
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
            let time: f64 = cfg_match! {
                feature = "std_web" => ({
                    match v {
                        Value::Number(n) => n.try_into().unwrap(),
                        _ => 0.0,
                    }
                }),
                feature = "web_sys" => v.as_f64().unwrap_or(0.),
            };
            callback.emit(time);
        };
        let handle = cfg_match! {
            feature = "std_web" => js! {
                var callback = @{callback};
                var action = function(time) {
                    callback(time);
                    callback.drop();
                };
                return {
                    render_id: requestAnimationFrame(action),
                    callback: callback,
                };
            },
            feature = "web_sys" => ({
                let callback = Closure::wrap(Box::new(callback) as Box<dyn Fn(JsValue)>);
                let render_id = utils::window().request_animation_frame(callback.as_ref().unchecked_ref()).unwrap();
                RenderTaskInner {
                    render_id,
                    callback,
                }
            }),
        };
        RenderTask(handle)
    }
}

impl Task for RenderTask {
    fn is_active(&self) -> bool {
        true
    }
}

impl Drop for RenderTask {
    fn drop(&mut self) {
        if self.is_active() {
            cfg_match! {
                feature = "std_web" => ({
                    let handle = &self.0;
                    js! { @(no_return)
                        var handle = @{handle};
                        cancelAnimationFrame(handle.render_id);
                        handle.callback.drop();
                    }
                }),
                feature = "web_sys" => utils::window().cancel_animation_frame(self.0.render_id).unwrap(),
            }
        }
    }
}
