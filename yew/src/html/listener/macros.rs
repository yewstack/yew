#[macro_use]
macro_rules! impl_action {
    ($($action:ident($type:ident) -> $ret:path => $convert:expr)*) => {$(
        impl_action!($action($type, false) -> $ret => $convert);
    )*};
    ($($action:ident($type:ident, $passive:literal) -> $ret:path => $convert:expr)*) => {$(
        /// An abstract implementation of a listener.
        #[doc(hidden)]
        pub mod $action {
            use cfg_if::cfg_if;
            use crate::callback::Callback;
            #[allow(unused_imports)]
            use crate::html::listener::*;
            use crate::virtual_dom::Listener;
            cfg_if! {
                if #[cfg(feature = "std_web")] {
                    use stdweb::web::event::$type;
                    use stdweb::web::{Element, IEventTarget};
                } else if #[cfg(feature = "web_sys")] {
                    use web_sys::Element;
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

            #[cfg(feature = "std_web")]
            type Argument = stdweb::Reference;
            #[cfg(feature = "web_sys")]
            type Argument = web_sys::Event;

            impl Listener for Wrapper {
                fn kind(&self) -> &'static str {
                    stringify!($action)
                }

                fn attach(&self, body: &Element, handler: Box<dyn Fn(Argument)>) {
                    #[cfg(feature = "std_web")]
                    body.add_event_listener(move |event: $type| handler(event.into()));

                    #[cfg(feature = "web_sys")]
                    crate::html::listener::macros::attach(
                        &self.kind()[2..],
                        body,
                        handler,
                        self.passive(),
                    );
                }

                fn handle(&self, event: Argument) {
                    self.callback.emit($convert(event));
                }

                #[cfg(feature = "web_sys")]
                fn passive(&self) -> bool {
                    match &self.callback {
                        Callback::CallbackWithOpts{passive, ..} => *passive,
                        _ => $passive,
                    }
                }

                fn handle_bubbled(&self) -> bool {
                    match &self.callback {
                        Callback::CallbackWithOpts{handle_bubbled, ..} => *handle_bubbled,
                        _ => false,
                    }
                }
            }
        }
    )*};
}

// Moved out to reduce instruction bloat
#[cfg(feature = "web_sys")]
pub(crate) fn attach(
    kind: &str,
    body: &web_sys::Element,
    handler: Box<dyn Fn(web_sys::Event)>,
    passive: bool,
) {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    let cl = Closure::wrap(handler);
    AsRef::<web_sys::EventTarget>::as_ref(body)
        .add_event_listener_with_callback_and_add_event_listener_options(
            &kind[2..],
            cl.as_ref().unchecked_ref(),
            &{
                let mut opts = web_sys::AddEventListenerOptions::new();
                opts.passive(passive);
                opts
            },
        )
        .map_err(|e| format!("could not register global listener: {:?}", e))
        .unwrap();
    cl.forget(); // Never drop the closure as this event handler is static
}
