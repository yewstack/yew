//! Gathering of items for compatibility between web-sys/js-sys and stdweb.

use ::{
    std::mem::ManuallyDrop,
    wasm_bindgen::{closure::Closure, JsCast},
    web_sys::EventTarget,
};

pub struct EventListenerHandle<T> {
    target: EventTarget,
    r#type: &'static str,
    callback: ManuallyDrop<Closure<dyn Fn(T)>>,
}

impl<T> EventListenerHandle<T> {
    pub fn remove(self) {
        self.target.remove_event_listener_with_callback(
            &self.r#type,
            self.callback.as_ref().unchecked_ref(),
        );
        let _ = ManuallyDrop::into_inner(self.callback);
    }
}
