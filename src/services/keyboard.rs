//! Service to register key press event listeners on elements.

use crate::callback::Callback;
use cfg_if::cfg_if;
use cfg_match::cfg_match;
use std::fmt;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::web::event::{ConcreteEvent, KeyDownEvent, KeyPressEvent, KeyUpEvent};
        use stdweb::web::{EventListenerHandle, IEventTarget};
    } else if #[cfg(feature = "web_sys")] {
        use gloo::events::EventListener;
        use wasm_bindgen::JsCast;
        use web_sys::{Event, EventTarget, KeyboardEvent};
    }
}

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
pub struct KeyListenerHandle(
    #[cfg(feature = "std_web")] Option<EventListenerHandle>,
    #[cfg(feature = "web_sys")] EventListener,
);

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
    pub fn register_key_press<
        #[cfg(feature = "std_web")] T: IEventTarget,
        #[cfg(feature = "web_sys")] T: AsRef<EventTarget>,
    >(
        element: &T,
        #[cfg(feature = "std_web")] callback: Callback<KeyPressEvent>,
        #[cfg(feature = "web_sys")] callback: Callback<KeyboardEvent>,
    ) -> KeyListenerHandle {
        cfg_match! {
            feature = "std_web" => register_key_impl(element, callback),
            feature = "web_sys" => register_key_impl(element, callback, "keypress"),
        }
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
    pub fn register_key_down<
        #[cfg(feature = "std_web")] T: IEventTarget,
        #[cfg(feature = "web_sys")] T: AsRef<EventTarget>,
    >(
        element: &T,
        #[cfg(feature = "std_web")] callback: Callback<KeyDownEvent>,
        #[cfg(feature = "web_sys")] callback: Callback<KeyboardEvent>,
    ) -> KeyListenerHandle {
        cfg_match! {
            feature = "std_web" => register_key_impl(element, callback),
            feature = "web_sys" => register_key_impl(element, callback, "keydown"),
        }
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
    pub fn register_key_up<
        #[cfg(feature = "std_web")] T: IEventTarget,
        #[cfg(feature = "web_sys")] T: AsRef<EventTarget>,
    >(
        element: &T,
        #[cfg(feature = "std_web")] callback: Callback<KeyUpEvent>,
        #[cfg(feature = "web_sys")] callback: Callback<KeyboardEvent>,
    ) -> KeyListenerHandle {
        cfg_match! {
            feature = "std_web" => register_key_impl(element, callback),
            feature = "web_sys" => register_key_impl(element, callback, "keyup"),
        }
    }
}

#[cfg(feature = "std_web")]
fn register_key_impl<T: IEventTarget, E: 'static + ConcreteEvent>(
    element: &T,
    callback: Callback<E>,
) -> KeyListenerHandle {
    let handle = element.add_event_listener(move |event: E| {
        callback.emit(event);
    });
    cfg_match! {
        feature = "std_web" => KeyListenerHandle(Some(handle)),
        feature = "web_sys" => KeyListenerHandle(handle),
    }
}

#[cfg(feature = "web_sys")]
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

    KeyListenerHandle(EventListener::new(element.as_ref(), event, listener))
}

#[cfg(feature = "std_web")]
impl Drop for KeyListenerHandle {
    fn drop(&mut self) {
        if let Some(handle) = self.0.take() {
            handle.remove()
        } else {
            panic!("Tried to drop KeyListenerHandle twice")
        }
    }
}
