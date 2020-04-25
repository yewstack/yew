#[macro_use]
macro_rules! impl_action {
    ($($action:ident(event: $type:ident) -> $ret:ty => $convert:expr)*) => {$(
        impl_action!($action(name: "", event: $type) -> $ret => $convert);
    )*};
    ($($action:ident(name: $name:literal, event: $type:ident) -> $ret:ty => $convert:expr)*) => {$(
        /// An abstract implementation of a listener.
        #[doc(hidden)]
        pub mod $action {
            use cfg_if::cfg_if;
            use cfg_match::cfg_match;
            use crate::callback::Callback;
            #[allow(unused_imports)]
            use crate::html::listener::*;
            use crate::virtual_dom::Listener;
            cfg_if! {
                if #[cfg(feature = "std_web")] {
                    use stdweb::web::event::{$type, IEvent};
                    use stdweb::web::{Element, IEventTarget};
                } else if #[cfg(feature = "web_sys")] {
                    use gloo::events::{EventListener, EventListenerOptions};
                    use wasm_bindgen::JsValue;
                    use web_sys::{$type as WebSysType, Element, EventTarget};
                }
            }

            /// A wrapper for a callback which attaches event listeners to elements.
            #[derive(Clone, Debug)]
            pub struct Wrapper {
                callback: Callback<Event>,
            }

            impl Wrapper {
                /// Create a wrapper for an event-typed callback
                pub fn new(callback: Callback<Event>) -> Self {
                    Wrapper { callback }
                }
            }

            /// And event type which keeps the returned type.
            pub type Event = $ret;

            impl Listener for Wrapper {
                fn kind(&self) -> &'static str {
                    stringify!($action)
                }

                fn attach(&self, element: &Element) -> EventListener {
                    let this = element.clone();
                    let callback = self.callback.clone();
                    let listener = move |
                        #[cfg(feature = "std_web")] event: $type,
                        #[cfg(feature = "web_sys")] event: &web_sys::Event
                    | {
                        event.stop_propagation();
                        #[cfg(feature = "web_sys")]
                        let event: WebSysType = JsValue::from(event).into();
                        callback.emit($convert(&this, event));
                    };
                    cfg_match! {
                        feature = "std_web" => EventListener(Some(element.add_event_listener(listener))),
                        feature = "web_sys" => ({
                            // We should only set passive event listeners for `touchstart` and `touchmove`.
                            // See here: https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener#Improving_scrolling_performance_with_passive_listeners
                            if $name == "touchstart" || $name == "touchmove" {
                                EventListener::new(&EventTarget::from(element.clone()), $name, listener)
                            } else {
                                let options = EventListenerOptions::enable_prevent_default();
                                EventListener::new_with_options(&EventTarget::from(element.clone()), $name, options, listener)
                            }
                        }),
                    }
                }
            }
        }
    )*};
}
