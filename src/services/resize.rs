//! This module contains the implementation of a service that listens for browser window resize events.
use std::fmt;
#[cfg(feature = "std_web")]
use stdweb::{
    js,
    web::{window, Window},
    Value,
};
use yew::callback::Callback;
#[cfg(feature = "web_sys")]
use ::{
    gloo::events::EventListener,
    web_sys::{Event, Window},
};

/// A service that fires events when the browser window resizes.
#[derive(Default, Debug)]
pub struct ResizeService {}

/// A handle to the event listener for resize events.
#[must_use]
pub struct ResizeTask(
    #[cfg(feature = "std_web")] Option<Value>,
    #[cfg(feature = "web_sys")] EventListener,
);

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
        let width = window.inner_width();
        let height = window.inner_height();
        #[cfg(feature = "web_sys")]
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
    /// Creates a new ResizeService.
    pub fn new() -> ResizeService {
        ResizeService {}
    }

    /// Register a callback that will be called when the browser window resizes.
    pub fn register(&mut self, callback: Callback<WindowDimensions>) -> ResizeTask {
        let callback = move |#[cfg(feature = "web_sys")] _event: &Event| {
            #[cfg(feature = "std_web")]
            let window = window();
            #[cfg(feature = "web_sys")]
            let window = web_sys::window().unwrap();
            let dimensions = WindowDimensions::get_dimensions(&window);
            callback.emit(dimensions);
        };
        #[cfg(feature = "std_web")]
        let handle = Some(js! {
            var callback = @{callback};
            var action = function() {
                callback();
            };
            return window.addEventListener("resize", action);
        });
        #[cfg(feature = "web_sys")]
        let handle = EventListener::new(&web_sys::window().unwrap(), "resize", callback);
        ResizeTask(handle)
    }
}

#[cfg(feature = "std_web")]
impl Drop for ResizeTask {
    fn drop(&mut self) {
        let handle = self.0.take().expect("Resize task already empty.");
        js! {
            @(no_return)
            var handle = @{handle};
            handle.callback.drop();
        }
    }
}
