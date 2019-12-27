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
            #[cfg(feature = "stdweb")]
            use stdweb::web::{
                event::{$type, IEvent},
                Element, EventListenerHandle, IEventTarget,
            };
            #[cfg(feature = "web_sys")]
            use ::{
                std::mem::ManuallyDrop,
                wasm_bindgen::{closure::Closure, JsCast},
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

                #[cfg(feature = "stdweb")]
                fn attach(&self, element: &Element) -> EventListenerHandle {
                    let this = element.clone();
                    let callback = self.callback.clone();
                    let listener = move |event: $type| {
                        event.stop_propagation();
                        callback.emit($convert(&this, event));
                    };
                    element.add_event_listener(listener)
                }

                #[cfg(feature = "web_sys")]
                fn attach(&self, element: &Element) -> EventListenerHandle {
                    let this = element.clone();
                    let callback = self.callback.clone();
                    let listener = move |event: web_sys::Event| {
                        event.stop_propagation();
                        let event: WebSysType = event.dyn_into().expect("wrong event type");
                        callback.emit($convert(&this, event));
                    };

                    let target = EventTarget::from(element.clone());
                    let listener = Closure::wrap(Box::new(listener) as Box<dyn Fn(web_sys::Event)>);
                    target
                        .add_event_listener_with_callback($name, listener.as_ref().unchecked_ref())
                        .expect("failed to add event listener");

                    EventListenerHandle {
                        target,
                        r#type: $name,
                        callback: ManuallyDrop::new(listener),
                    }
                }
            }
        }
    )*};
}
