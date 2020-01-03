#[macro_use]
macro_rules! impl_action {
    ($($action:ident(event: $type:ident) -> $ret:ty => $convert:expr)*) => {$(
        impl_action!($action(name: "", event: $type) -> $ret => $convert);
    )*};
    ($($action:ident(name: $name:literal, event: $type:ident) -> $ret:ty => $convert:expr)*) => {$(
        /// An abstract implementation of a listener.
        pub mod $action {
            use crate::callback::Callback;
            #[allow(unused_imports)]
            use crate::html::listener::*;
            use crate::virtual_dom::Listener;
            #[cfg(feature = "std_web")]
            use stdweb::web::{
                event::{$type, IEvent},
                Element, EventListenerHandle, IEventTarget,
            };
            #[cfg(feature = "web_sys")]
            use ::{
                wasm_bindgen::JsValue,
                web_sys::{$type as WebSysType, Element, EventTarget},
            };

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

                fn attach(&self, element: &Element) -> EventListenerHandle {
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
                    #[cfg(feature = "std_web")]
                    {
                        element.add_event_listener(listener)
                    }
                    #[cfg(feature = "web_sys")]
                    EventListenerHandle::new(&EventTarget::from(element.clone()), $name, listener)
                }
            }
        }
    )*};
}
