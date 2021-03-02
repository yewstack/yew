//! Service to register key press event listeners on elements.

use gloo::events::{EventListener, EventListenerOptions};
use std::fmt;
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, KeyboardEvent};
use yew::callback::Callback;

/// Service for registering callbacks on elements to get keystrokes from the user.
///
/// # Note
/// Elements which natively support keyboard input (such as `<input/>` or `<textarea/>`) can use the
/// `onkeypress` or `oninput` attributes from within the html macro. You **should use those events
/// instead** of locating the element and registering an event listener using this service.
///
/// This service is for adding key event listeners to elements which don't support these attributes,
/// (for example the `document` and `<canvas>` elements).
#[derive(Debug)]
pub struct KeyboardService {}

/// Handle for the key event listener.
///
/// When the handle goes out of scope, the listener will be removed from the element.
#[must_use = "the listener is only active until the handle is dropped"]
pub struct KeyListenerHandle(EventListener);

impl fmt::Debug for KeyListenerHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("KeyListenerHandle")
    }
}

impl KeyboardService {
    /// Registers a callback which listens to KeyPressEvents on a provided element.
    ///
    /// # Documentation
    /// [keypress event](https://developer.mozilla.org/en-US/docs/Web/API/Document/keypress_event)
    ///
    /// # Warning
    /// This API has been deprecated in the HTML standard and it is not recommended for use in new projects.
    /// Consult the browser compatibility chart in the linked MDN documentation.
    pub fn register_key_press<T: AsRef<EventTarget>>(
        element: &T,
        callback: Callback<KeyboardEvent>,
    ) -> KeyListenerHandle {
        register_key_impl(element, callback, "keypress")
    }

    /// Registers a callback which listens to KeyDownEvents on a provided element.
    ///
    /// # Documentation
    /// [keydown event](https://developer.mozilla.org/en-US/docs/Web/API/Document/keydown_event)
    ///
    /// # Note
    /// This browser feature is relatively new and is set to replace the `keypress` event.
    /// It may not be fully supported in all browsers.
    /// Consult the browser compatibility chart in the linked MDN documentation.
    pub fn register_key_down<T: AsRef<EventTarget>>(
        element: &T,
        callback: Callback<KeyboardEvent>,
    ) -> KeyListenerHandle {
        register_key_impl(element, callback, "keydown")
    }

    /// Registers a callback that listens to KeyUpEvents on a provided element.
    ///
    /// # Documentation
    /// [keyup event](https://developer.mozilla.org/en-US/docs/Web/API/Document/keyup_event)
    ///
    /// # Note
    /// This browser feature is relatively new and is set to replace keypress events.
    /// It may not be fully supported in all browsers.
    /// Consult the browser compatibility chart in the linked MDN documentation.
    pub fn register_key_up<T: AsRef<EventTarget>>(
        element: &T,
        callback: Callback<KeyboardEvent>,
    ) -> KeyListenerHandle {
        register_key_impl(element, callback, "keyup")
    }
}

fn register_key_impl<T: AsRef<EventTarget>>(
    element: &T,
    callback: Callback<KeyboardEvent>,
    event: &'static str,
) -> KeyListenerHandle {
    let listener = move |event: &Event| {
        let event = event
            .dyn_ref::<KeyboardEvent>()
            .expect("wrong event type")
            .clone();
        callback.emit(event);
    };
    let options = EventListenerOptions::enable_prevent_default();
    KeyListenerHandle(EventListener::new_with_options(
        element.as_ref(),
        event,
        options,
        listener,
    ))
}
