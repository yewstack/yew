//! Developer tools.
//! These communicate with a backend over a WebSocket connection.
//! Messages are sent as JSON.

use serde::Serialize;
use serde_json;

#[cfg(feature = "web_sys")]
use web_sys;

#[cfg(feature = "std_web")]
use stdweb;

pub mod messages;

/// Stores a connection to the DevTools server.
#[derive(Debug, Clone)]
pub struct DebuggerConnection {
    #[cfg(feature = "web_sys")]
    ws: web_sys::WebSocket,
    #[cfg(feature = "std_web")]
    ws: stdweb::web::WebSocket,
    message_queue: Vec<String>
}

/// A debugger is capable of sending messages over a WebSocket connection.
pub trait Debugger<T>
where
    T: Serialize,
{
    /// Queue a message to be sent.
    fn queue_message(&mut self, message: T);

}

impl<T: Serialize> Debugger<T> for DebuggerConnection {
    fn queue_message(&mut self, message: T) {
        self.message_queue.push(serde_json::to_string(&message).unwrap());
    }
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
                    panic!("Could not open a connection to the DevTools WebSocket.");
                }
            },
            #[cfg(feature = "std_web")]
            ws: match stdweb::web::WebSocket::new(&ws_url) {
                Ok(s) => s,
                Err(_) => {
                    stdweb::console!(error, "Error: could not open a connection to the DevTools WebSocket. Are you sure the DevTools backend is running?");
                    panic!("Could not open a connection to the DevTools WebSocket.")
                }
            },
            message_queue: Vec::new()
        }
    }
}

#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;
    #[wasm_bindgen_test]
    fn test_message_queuing() {
        struct TestComponent {}
        impl crate::Component for TestComponent {
            type Message = ();
            type Properties = ();
            fn create(_: Self::Properties, _l: crate::ComponentLink<Self>) -> Self {
                Self {}
            }
            fn change(&mut self, _props: Self::Properties) -> bool {
                false
            }
            fn update(&mut self, _: Self::Message) -> bool {
                false
            }
            fn view(&self) -> crate::Html {
                html!(
                    <h1>{"Hello World!"}</h1>
                )
            }
        }
        let app: crate::App<TestComponent> = crate::App::new();
        app.mount(crate::utils::document().get_element_by_id("output").unwrap());
        crate::DEBUGGER_CONNECTION.with(|debugger| {
            // should have been created and mounted only
            assert_eq!(debugger.borrow().message_queue.len(), 2)
        });
    }
}
