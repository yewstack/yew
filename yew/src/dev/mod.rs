//! Developer tools.
//! These communicate with a backend over a WebSocket connection.
//! Messages are sent as JSON.

use serde::Serialize;
use serde_json;

#[cfg(feature = "web_sys")]
use web_sys;

use cfg_if::cfg_if;

#[cfg(feature = "std_web")]
use stdweb;

pub mod messages;

/// Stores a connection to the DevTools server.
#[derive(Debug)]
pub struct DebuggerConnection {
    #[cfg(feature = "web_sys")]
    ws: web_sys::WebSocket,
    #[cfg(feature = "std_web")]
    ws: stdweb::web::WebSocket,
}

/// A debugger is capable of sending messages over a WebSocket connection.
pub trait Debugger<T>
where
    T: Serialize,
{
    /// Sends a message over websockets.
    fn send_message(&self, message: T);
}

impl DebuggerConnection {
    /// Creates a new connection to the debugger.
    /// The URL to which the debugger attempts to connect can be configured by setting some environment variables at compile time.
    /// If you do not set any of these environment variables, the default values are used.
    /// The following variables are accepted: `YEW_DEBUGGER_CONNECTION_TYPE`, `YEW_DEBUGGER_HOST` and `YEW_DEBUGGER_PORT`.
    /// * `YEW_DEBUGGER_CONNECTION_TYPE` – either `ws` or `wss`. `ws` is an insecure WebSocket (but this is fine for local development) and `wss` creates a secure WebSocket which can be used for remote debugging. You will need to set up certificates for a secure connection.
    /// * `YEW_DEBUGGER_HOST` – a domain or IP address where a debug server can be found.
    /// * `YEW_DEBUGGER_PORT` – the port on which the debugger is operating.
    pub fn new() -> Self {
        let ws_url = format!(
            "{}://{}:{}",
            match std::option_env!("YEW_DEBUGGER_CONNECTION_TYPE") {
                Some(ws_mode) => {
                    match ws_mode {
                        "ws" => "ws",
                        "wss" => "wss",
                        _ => {
                            panic!("`{}` is not a valid option for the `YEW_DEBUGGER_SECURE_CONNECTION` environment variable. Use either `ws` for an insecure connection or `wss` for a secure connection.", ws_mode);
                        }
                    }
                }
                None => {
                    "ws"
                }
            },
            match std::option_env!("YEW_DEBUGGER_HOST") {
                Some(url) => {
                    url
                }
                None => {
                    "localhost"
                }
            },
            match std::option_env!("YEW_DEBUGGER_PORT") {
                Some(port) => {
                    port
                }
                None => {
                    "8017"
                }
            }
        );
        Self {
            #[cfg(feature = "web_sys")]
            ws: match web_sys::WebSocket::new(&ws_url) {
                Ok(s) => s,
                Err(_) => {
                    web_sys::console::error_1(&"Error: could not open a connection to the DevTools WebSocket. Are you sure the DevTools backend is running?".into());
                    panic!("");
                }
            },
            #[cfg(feature = "std_web")]
            ws: match stdweb::web::WebSocket::new(&ws_url) {
                Ok(s) => s,
                Err(_) => {
                    stdweb::console!(error, "Error: could not open a connection to the DevTools WebSocket. Are you sure the DevTools backend is running?");
                    panic!("")
                }
            },
        }
    }
}

impl<T: Serialize> Debugger<T> for DebuggerConnection {
    fn send_message(&self, message: T) {
        cfg_if! {
            if #[cfg(feature="web_sys")] {
                match self.ws.send_with_str(&serde_json::to_string(&message).unwrap()) {
                    Ok(_) => {}
                    Err(e) => println!("Error sending debug message: {:?}", e),
                };
            }
            else {
                match self.ws.send_text(&serde_json::to_string(&message).unwrap()) {
                    Ok(_) => {},
                    Err(e) => println!("Error sending debug message: {:?}", e)
                };
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use cfg_if::cfg_if;
    use cfg_match::cfg_match;
    use wasm_bindgen_test::*;
    use wasm_bindgen::prelude::*;
    #[cfg(feature = "wasm_test")]
    #[wasm_bindgen_test]
    fn test_websocket_logging() {
        use crate::html::{Component, ComponentLink, Html};
        cfg_if! {
            if #[cfg(feature="std_web")] {
                use stdweb::traits::IMessageEvent;
                use stdweb::web::event::SocketMessageEvent;
            } else if #[cfg(feature="web_sys")] {
                use gloo::events::EventListener;
                use web_sys::{Event, MessageEvent};
                use wasm_bindgen::JsCast;
            }
        }

        wasm_bindgen_test_configure!(run_in_browser);

        struct TestDebugComponent {}

        impl Component for TestDebugComponent {
            type Message = ();
            type Properties = ();
            fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
                Self {}
            }
            fn change(&mut self, _props: Self::Properties) -> bool {
                unimplemented!()
            }
            fn update(&mut self, _: Self::Message) -> bool {
                false
            }
            fn view(&self) -> Html {
                html!(
                    <>
                    <h1>{"Hello World!"}</h1>
                    <p>{"HELLO WORLD2"}</p>
                    </>
                )
            }
        }
        let test_debug_app: crate::App<TestDebugComponent> = crate::App::new();
        test_debug_app.mount(
            crate::utils::document()
                .get_element_by_id("output")
                .unwrap(),
        );
        #[cfg(feature="web_sys")]
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            let response = e
                .data()
                .as_string();
            match response {
                Some(text) => {
                    web_sys::console::log_1(&text.into());
                },
                None => {

                }
            };
        }) as Box<dyn FnMut(MessageEvent)>);
        #[cfg(feature="web_sys")]
        crate::DEBUGGER_CONNECTION.with(|d| {
            d.ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        });
        #[cfg(feature="web_sys")]
        onmessage_callback.forget();
    }

}
