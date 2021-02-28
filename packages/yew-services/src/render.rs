//! This module contains Yew's implementation of a service which can be used to
//! request frame rendering

use crate::Task;
use std::fmt;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use yew::callback::Callback;
use yew::utils;

/// A handle to cancel a render task.
#[must_use = "the task will be cancelled when the task is dropped"]
pub struct RenderTask(RenderTaskInner);

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
    /// Request animation frame. Callback will be notified when frame should be rendered.
    pub fn request_animation_frame(callback: Callback<f64>) -> RenderTask {
        let callback = move |v: JsValue| {
            let time: f64 = v.as_f64().unwrap_or(0.);
            callback.emit(time);
        };
        let handle = {
            let callback = Closure::wrap(Box::new(callback) as Box<dyn Fn(JsValue)>);
            let render_id = utils::window()
                .request_animation_frame(callback.as_ref().unchecked_ref())
                .unwrap();
            RenderTaskInner {
                render_id,
                callback,
            }
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
            utils::window()
                .cancel_animation_frame(self.0.render_id)
                .unwrap()
        }
    }
}
