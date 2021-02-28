//! This module contains Yew's implementation of a service which listens for browser window resize events.
use crate::Task;
use gloo::events::EventListener;
use std::fmt;
use web_sys::{Event, Window};
use yew::callback::Callback;

/// A service which fires events when the browser window is resized.
#[derive(Default, Debug)]
pub struct ResizeService {}

/// A handle for the event listener listening for resize events.
#[must_use = "the listener is only active until the task is dropped"]
pub struct ResizeTask(EventListener);

impl fmt::Debug for ResizeTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ResizeTask")
    }
}

/// Dimensions of the window.
#[derive(Copy, Clone, Debug)]
pub struct WindowDimensions {
    /// The width of the viewport of the browser window.
    pub width: i32,
    /// The height of the viewport of the browser window.
    pub height: i32,
}

impl WindowDimensions {
    /// Gets the dimensions of the browser window.
    pub fn get_dimensions(window: &Window) -> Self {
        let width = window.inner_width();
        let height = window.inner_height();
        let (width, height) = {
            (
                width.unwrap().as_f64().unwrap() as _,
                height.unwrap().as_f64().unwrap() as _,
            )
        };
        WindowDimensions { width, height }
    }
}

impl ResizeService {
    /// Register a callback that will be called when the browser window resizes.
    pub fn register(callback: Callback<WindowDimensions>) -> ResizeTask {
        let callback = move |_event: &Event| {
            let window = web_sys::window().unwrap();
            let dimensions = WindowDimensions::get_dimensions(&window);
            callback.emit(dimensions);
        };
        let handle = EventListener::new(&web_sys::window().unwrap(), "resize", callback);
        ResizeTask(handle)
    }
}

impl Task for ResizeTask {
    fn is_active(&self) -> bool {
        true
    }
}
