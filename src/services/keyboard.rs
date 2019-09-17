use stdweb::web::{document, IEventTarget, EventListenerHandle};
use stdweb::web::event::KeyPressEvent;

/// Service for registering callbacks on elements to get keystrokes from the user.
///
/// # Note
/// Elements that natively support keyboard input (input or textarea) can set an
/// `onkeypress` or `oninput` attribute within the html macro. You **should** use those events instead of
/// locating the element and registering it with this service.
///
/// This service is for adding key event listeners to elements that don't support these attributes,
/// like the `document` or `<canvas>` elements for example.
pub struct KeyboardService {}

/// Handle to the key event listener.
///
/// When it goes out of scope, the listener will be removed from the element.
pub struct KeyListenerHandle(EventListenerHandle);

impl KeyboardService {
    /// Registers a callback that listens to KeyPressEvents on a provided element.
    pub fn register(element: &IEventTarget, callback: Callback<String>) -> KeyListenerHandle {
        let handle = element.add_event_listener(move |event: KeyPressEvent| {
            let key = event.key();
            callback.emit(key);
        });
        KeyListenerHandle(handle)
    }
}

impl Drop for KeyListenerHandle {
    fn drop(&mut self) {
        self.0.remove()
    }
}