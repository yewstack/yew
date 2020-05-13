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
    /// A list of all the messages to be sent.
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

/// A debugger is capable of sending messages over a WebSocket connection.
pub trait DebuggerMessageQueue<T>
where
    T: Serialize,
{
    /// Adds a message to the message queue.
    fn queue_message(&mut self, message: T);
}

/// Sends messages.
pub trait DebuggerMessageSend<T>
where
    T: Serialize,
{
    /// Send all the messages in the queue.
    fn send_messages(&mut self);
    fn send_message(&mut self, message: T);
}

impl<T: Serialize> DebuggerMessageQueue<T> for DebuggerConnection {
    fn queue_message(&mut self, message: T) {
        self.message_queue
            .push(serde_json::to_string(&message).unwrap());
    }
}

impl<T: Serialize> DebuggerMessageSend<T> for DebuggerConnection {
    fn send_messages(&mut self) {
        for _ in 1..self.message_queue.len() {
            // either send message if the websocket is open or else use a promise
            match self.ws.ready_state() {
                // still connecting, so add an event listener which sends the messages once connected
                0 => {
                    let mut this = self.clone();
                    let ws_listener = move |_: &web_sys::Event| this.send_messages();
                    gloo::events::EventListener::new(&self.ws, "open", ws_listener);
                }
                1 => {
                    for _ in 1..self.message_queue.len() {
                        self.ws.send_with_str(&self.message_queue.pop().unwrap());
                    }
                }
                2 | 3 =>
                {
                    #[cfg(feature = "web_sys")]
                    web_sys::console::error_1(
                        &"Could not open a connection to Yew's developer tools; are they running?"
                            .into(),
                    )
                }
                _ => panic!("The WebSocket is in an incorrect state."),
            }
        }
    }
    fn send_message(&mut self, message: T) {
        match self.ws.ready_state() {
            0 => {
                self.queue_message(message);
                self.send_messages();
            }
            1 => {
                self.ws
                    .send_with_str(serde_json::to_string(&message).unwrap());
            }
            2 | 3 =>
            {
                #[cfg(feature = "web_sys")]
                web_sys::console::error_1(
                    &"Could not open a connection to Yew's developer tools; are they running?"
                        .into(),
                )
            }
            _ => panic!("The WebSocket is in an incorrect state."),
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
            std::option_env!("YEW_DEBUGGER_HOST")
                .as_deref()
                .unwrap_or("localhost"),
            std::option_env!("YEW_DEBUGGER_PORT")
                .as_deref()
                .unwrap_or("8017")
        );
        Self {
            #[cfg(feature = "web_sys")]
            ws: match web_sys::WebSocket::new(&ws_url) {
                Ok(s) => {
                    gloo::events::EventListener::new(&s, "close", |_| {
                        web_sys::console::error_1(&"Error: could not open a connection to the DevTools WebSocket. Are you sure the DevTools backend is running?".into());
                        panic!("Could not open a connection to the DevTools WebSocket.");
                    });
                    return s;
                }
                Err(_) => {
                    panic!("");
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
    use wasm_bindgen_test::*;
}
