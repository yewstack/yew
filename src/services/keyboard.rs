//! Service to register key press event listeners on elements.
use crate::callback::Callback;
use std::fmt;
use stdweb::web::event::{KeyDownEvent, KeyPressEvent, KeyUpEvent};
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
#[derive(Debug)]
pub struct KeyboardService {}

/// Handle to the key event listener.
///
/// When it goes out of scope, the listener will be removed from the element.
pub struct KeyListenerHandle(Option<EventListenerHandle>);

impl fmt::Debug for KeyListenerHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("KeyListenerHandle")
    }
}

impl KeyboardService {
    /// Registers a callback that listens to KeyPressEvents on a provided element.
    ///
    /// # Documentation
    /// [keypress event](https://developer.mozilla.org/en-US/docs/Web/API/Document/keypress_event)
    ///
    /// # Warning
    /// This API has been deprecated in the HTML standard and it is not recommended for use in new projects.
    /// Consult with the browser compatibility chart in the linked MDN documentation.
    pub fn register_key_press<T: IEventTarget>(
        element: &T,
        callback: Callback<KeyPressEvent>,
    ) -> KeyListenerHandle {
        let handle = element.add_event_listener(move |event: KeyPressEvent| {
            callback.emit(event);
        });
        KeyListenerHandle(Some(handle))
    }

    /// Registers a callback that listens to KeyDownEvents on a provided element.
    ///
    /// # Documentation
    /// [keydown event](https://developer.mozilla.org/en-US/docs/Web/API/Document/keydown_event)
    ///
    /// # Note
    /// This browser feature is relatively new and is set to replace keypress events.
    /// Not all browsers may support it completely.
    /// Consult with the browser compatibility chart in the linked MDN documentation.
    pub fn register_key_down<T: IEventTarget>(
        element: &T,
        callback: Callback<KeyDownEvent>,
    ) -> KeyListenerHandle {
        let handle = element.add_event_listener(move |event: KeyDownEvent| {
            callback.emit(event);
        });
        KeyListenerHandle(Some(handle))
    }

    /// Registers a callback that listens to KeyUpEvents on a provided element.
    ///
    /// # Documentation
    /// [keyup event](https://developer.mozilla.org/en-US/docs/Web/API/Document/keyup_event)
    ///
    /// # Note
    /// This browser feature is relatively new and is set to replace keypress events.
    /// Not all browsers may support it completely.
    /// Consult with the browser compatibility chart in the linked MDN documentation.
    pub fn register_key_up<T: IEventTarget>(
        element: &T,
        callback: Callback<KeyUpEvent>,
    ) -> KeyListenerHandle {
        let handle = element.add_event_listener(move |event: KeyUpEvent| {
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
