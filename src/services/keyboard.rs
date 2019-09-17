//! Service to register key press event listeners on elements.
use crate::callback::Callback;
use stdweb::web::event::KeyPressEvent;
use stdweb::web::{EventListenerHandle, IEventTarget};

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
pub struct KeyListenerHandle(Option<EventListenerHandle>);



impl KeyboardService {
    /// Registers a callback that listens to KeyPressEvents on a provided element.
    pub fn register<T: IEventTarget>(element: &T, callback: Callback<KeyPressEvent>) -> KeyListenerHandle {
        let handle = element.add_event_listener(move |event: KeyPressEvent| {
            callback.emit(event);
        });
        KeyListenerHandle(Some(handle))
    }
}

impl Drop for KeyListenerHandle {
    fn drop(&mut self) {
        if let Some(handle) = self.0.take() {
            handle.remove()
        } else {
            panic!("Tried to drop KeyListenerHandle twice")
        }
    }
}
