//! Gathering of items for compatibility between web-sys/js-sys and stdweb.

use std::mem::ManuallyDrop;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Event, EventTarget};

pub struct EventListenerHandle {
    target: EventTarget,
    r#type: &'static str,
    callback: ManuallyDrop<Closure<dyn Fn(Event)>>,
}

impl EventListenerHandle {
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
