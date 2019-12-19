//! Gathering of items for compatibility between web-sys/js-sys and stdweb.

use std::mem::ManuallyDrop;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Event, EventTarget};

/// Handler to an event listener, only use is to cancel the event.
// We can't use `gloo`s implementation because it cancels the event upon dropping the handler, but
// we want the event to only be cancelled when the user desires. The main issue here is that
// `wasm-bindgen` doesn't support moving a closure to WASM, so the closure has to be "forgotten"
// and not be dropped, therefore the use of `ManuallyDrop` here.
#[derive(Debug)]
pub struct EventListenerHandle {
    pub(crate) target: EventTarget,
    pub(crate) r#type: &'static str,
    pub(crate) callback: ManuallyDrop<Closure<dyn Fn(Event)>>,
}

impl EventListenerHandle {
    /// Cancel event.
    pub fn remove(self) {
        self.target
            .remove_event_listener_with_callback(
                &self.r#type,
                self.callback.as_ref().unchecked_ref(),
            )
            .expect("failed to remove event listener");
        let _ = ManuallyDrop::into_inner(self.callback);
    }
}
