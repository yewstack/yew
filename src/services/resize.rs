use stdweb::Value;
use yew::callback::Callback;


/// A service that fires events when the browser window resizes.
#[derive(Default)]
pub struct ResizeService {}

/// A handle to the event listener for resize events.
#[must_use]
pub struct ResizeTask(Option<Value>);

impl ResizeService {
    /// Creates a new ResizeService
    pub fn new() -> ResizeService {
        ResizeService {}
    }

    /// Register a callback that will be called when the browser window resizes.
    pub fn register(&mut self, callback: Callback<()>) -> ResizeTask {
        let callback = move || {
            callback.emit(());
        };
        let handle = js! {
            var callback = @{callback};
            var action = function() {
                callback();
            };
            return window.addEventListener("resize", action);
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
            handle.callback.drop();
        }
    }
}
