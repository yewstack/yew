//! This module contains Yew's implementation of a service which listens for browser window resize events.
use cfg_if::cfg_if;
use cfg_match::cfg_match;
use std::fmt;
use yew::callback::Callback;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::js;
        use stdweb::web::{event::ResizeEvent, window, Window};
        use stdweb::Value;
    } else if #[cfg(feature = "web_sys")] {
        use gloo::events::EventListener;
        use web_sys::{Event, Window};
    }
}

/// A service which fires events when the browser window is resized.
#[derive(Default, Debug)]
pub struct ResizeService {}

/// A handle for the event listener listening for resize events.
#[must_use]
pub struct ResizeTask(
    #[cfg(feature = "std_web")] Value,
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
    /// Register a callback that will be called when the browser window resizes.
    pub fn register(callback: Callback<WindowDimensions>) -> ResizeTask {
        let callback =
            move |#[cfg(feature = "web_sys")] _event: &Event,
                  #[cfg(feature = "std_web")] _event: ResizeEvent| {
                let window = cfg_match! {
                    feature = "std_web" => window(),
                    feature = "web_sys" => web_sys::window().unwrap(),
                };
                let dimensions = WindowDimensions::get_dimensions(&window);
                callback.emit(dimensions);
            };
        let handle = cfg_match! {
            feature = "std_web" => js! {
                var handle = @{callback};
                window.addEventListener("resize", handle);
                return handle;
            },
            feature = "web_sys" => EventListener::new(&web_sys::window().unwrap(), "resize", callback),
        };
        ResizeTask(handle)
    }
}

#[cfg(feature = "std_web")]
impl Drop for ResizeTask {
    fn drop(&mut self) {
        let handle = &self.0;
        js! {
            @(no_return)
            var handle = @{handle};
            window.removeEventListener("resize", handle);
        }
    }
}
