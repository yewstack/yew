//! This module contains the implementation of a service that listens for browser window resize events.
use std::fmt;
use stdweb::Value;
use stdweb::{
    js,
    web::{window, Window},
};
use yew::callback::Callback;

/// A service that fires events when the browser window resizes.
#[derive(Default, Debug)]
pub struct ResizeService {}

/// A handle to the event listener for resize events.
#[must_use]
pub struct ResizeTask(Option<Value>);

impl fmt::Debug for ResizeTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ResizeTask")
    }
}

/// Dimensions of the window.
#[derive(Debug)]
pub struct WindowDimensions {
    /// The width of the viewport of the browser window.
    pub width: i32,
    /// The height of the viewport of the browser window.
    pub height: i32,
}

impl WindowDimensions {
    /// Gets the dimensions of the browser window.
    pub fn get_dimensions(window: &Window) -> Self {
        WindowDimensions {
            width: window.inner_width(),
            height: window.inner_height(),
        }
    }
}

impl ResizeService {
    /// Creates a new ResizeService.
    pub fn new() -> ResizeService {
        ResizeService {}
    }

    /// Register a callback that will be called when the browser window resizes.
    pub fn register(&mut self, callback: Callback<WindowDimensions>) -> ResizeTask {
        let callback = move || {
            let window = window();
            let dimensions = WindowDimensions::get_dimensions(&window);
            callback.emit(dimensions);
        };
        let handle = js! {
            var handle = @{callback};
            window.addEventListener("resize", handle);
            return handle;
        };
        ResizeTask(Some(handle))
    }
}

impl Drop for ResizeTask {
    fn drop(&mut self) {
        let handle = self.0.take().expect("Resize task already empty.");
        js! {
            @(no_return)
            var handle = @{handle};
            window.removeEventListener("resize", handle);
        }
    }
}
