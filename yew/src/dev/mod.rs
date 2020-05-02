//! Developer tools.
//! These communicate with a debugging server (normally a browser extension) over a WebSocket connection.

use serde::Serialize;
use serde_json;

#[cfg(test)]
pub use messages::tests as message_tests;

#[cfg(feature = "web_sys")]
use web_sys;

#[cfg(feature = "std_web")]
use stdweb;

pub mod messages;

/// Stores a connection to the DevTools server.
#[derive(Debug, Clone)]
pub struct DebuggerConnection {
    #[cfg(feature = "web_sys")]
    /// Public only for testing.
    pub ws: web_sys::WebSocket,
    #[cfg(feature = "std_web")]
    /// Public only for testing.
    pub ws: stdweb::web::WebSocket,
    message_queue: Vec<String>,
}

/// Describes the state of a WebSocket connection.
#[derive(Debug)]
pub enum ConnectionState {
    /// The socket has successfully connected.
    Connected,
    /// The socket has not successfully connnected.
    CouldntConnect,
}

impl std::future::Future for DebuggerConnection {
    type Output = Self;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match self.ws.ready_state() {
            0 => std::task::Poll::Pending,
            1 => std::task::Poll::Ready(self.clone()),
            2 => std::task::Poll::Pending,
            3 => std::task::Poll::Ready(self.clone()),
            _ => std::task::Poll::Pending,
        }
    }
}

/// A debugger is capable of sending messages over a WebSocket connection.
pub trait DebuggerMessageQueue<T>
where
    T: Serialize,
{
    /// Adds a message to the message queue.
    fn queue_message(&mut self, message: T);
}

/// Sends messages.
pub trait DebuggerMessageSend {
    /// Send all the messages in the queue.
    fn send_messages(&mut self);
}

impl<T: Serialize> DebuggerMessageQueue<T> for DebuggerConnection {
    fn queue_message(&mut self, message: T) {
        self.message_queue
            .push(serde_json::to_string(&message).unwrap());
    }
}

impl DebuggerMessageSend for DebuggerConnection {
    fn send_messages(&mut self) {
        for _ in 1..self.message_queue.len() {
            match self.ws.send_with_str(self.message_queue.first().unwrap()) {
                Ok(_) => {
                    self.message_queue.pop();
                }
                Err(_) => {}
            }
        }
    }
}

impl DebuggerConnection {
    /// Creates a new connection to the debugger.
    /// The URL to which the debugger attempts to connect can be configured by setting some environment variables at compile time.
    /// If you do not set any of these environment variables, the default values are used.
    /// The following variables are accepted: `YEW_DEBUGGER_CONNECTION_TYPE`, `YEW_DEBUGGER_HOST` and `YEW_DEBUGGER_PORT`.
    /// * `YEW_DEBUGGER_CONNECTION_TYPE` – either `ws` or `wss`. `ws` is an insecure WebSocket (but this is fine for local development) and `wss` creates a secure WebSocket which can be used for remote debugging. If you choose `wss` you will need to ensure that your server is configured correctly.
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
                    panic!("Could not open a connection to the DevTools WebSocket.");
                }
            },
            message_queue: Vec::new(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::dev::{DebuggerConnection, DebuggerMessageQueue, DebuggerMessageSend};
    use std::ops::DerefMut;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_messages_send() {
        let mut debugger = DebuggerConnection::new();
        debugger.queue_message(crate::dev::messages::DebugComponent::new(
            "Test".to_string(),
            None,
        ));
        assert_eq!(debugger.message_queue.len(), 1);
        wasm_bindgen_futures::spawn_local(async {
            let mut result = debugger.await;
            assert_eq!(result.message_queue.len(), 1);
            result.send_messages();
            assert_eq!(result.message_queue.len(), 0);
        });
    }

    #[wasm_bindgen_test]
    async fn test_integration() {
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
        app.mount(
            crate::utils::document()
                .get_element_by_id("output")
                .unwrap(),
        );
        let mut debugger = crate::DEBUGGER_CONNECTION.with(|debugger| {
            debugger.replace(crate::dev::DebuggerConnection::new())
        });
        wasm_bindgen_futures::spawn_local(async {
            let mut new = debugger.await;
            new.send_messages();
        });
    }
}
