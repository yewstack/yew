//! Developer tools.
//! The developer tools connect to a WebSocket server (running as a browser extension).

use serde::Serialize;

thread_local! {
    /// A global debugger object.
    pub static DEBUGGER_CONNECTION: std::cell::RefCell<DebuggerConnection>
    = std::cell::RefCell::new(DebuggerConnection::new());
}

#[cfg(test)]
pub use messages::tests as message_tests;

pub mod messages;

/// Stores a connection to the DevTools server.
#[derive(Debug, Clone)]
pub struct DebuggerConnection {
    /// Public only for testing.
    pub ws: web_sys::WebSocket,
    message_queue: Vec<String>,
    created_listener: bool,
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
pub trait DebuggerMessageFlush {
    /// Send all the messages in the queue.
    fn send_messages(&mut self);
}

/// Sends a single message, or adds it to the queue if the WebSocket is not open.
pub trait DebuggerMessageSend<T>
where
    T: Serialize,
{
    /// Send a message
    fn send_message(&mut self, message: T);
}

/// Adds a message to the queue.
impl<T: Serialize> DebuggerMessageQueue<T> for DebuggerConnection {
    /// Add a message to the queue.
    fn queue_message(&mut self, message: T) {
        self.message_queue
            .push(serde_json::to_string(&message).unwrap());
    }
}

/// Tries to send all the messages in the queue.
impl DebuggerMessageFlush for DebuggerConnection {
    /// Tries to send all the queued messages or adds an event listener if the connection is not yet open.
    fn send_messages(&mut self) {
        for _ in 1..self.message_queue.len() {
            // either send the messages if the websocket is open or create an event listener to send them when ready
            match self.ws.ready_state() {
                // still connecting, so add an event listener which sends the messages once connected
                0 => {
                    if !self.created_listener {
                        let ws_listener = move |_: &web_sys::Event| {
                            DEBUGGER_CONNECTION.with(|d| d.borrow_mut().send_messages())
                        };
                        std::mem::forget(gloo::events::EventListener::new(
                            &self.ws,
                            "open",
                            ws_listener,
                        ));
                        self.created_listener = true;
                    }
                }
                1 => {
                    for _ in 1..self.message_queue.len() {
                        self.ws.send_with_str(&self.message_queue.pop().unwrap());
                    }
                }
                2 | 3 => {
                    #[cfg(feature = "web_sys")]
                    {
                        web_sys::console::error_1(&"Error: could not open a connection to the DevTools WebSocket. Are you sure the DevTools backend is running?".into());
                        panic!("Could not open a connection to the DevTools WebSocket.");
                    };
                }
                _ => panic!("The WebSocket is in an incorrect state."),
            }
        }
    }
}

impl<T: Serialize> DebuggerMessageSend<T> for DebuggerConnection {
    fn send_message(&mut self, message: T) {
        match self.ws.ready_state() {
            0 => {
                self.queue_message(message);
            }
            1 => {
                match self
                    .ws
                    .send_with_str(&serde_json::to_string(&message).unwrap())
                {
                    Ok(_) => {}
                    Err(e) => web_sys::console::error_1(
                        &format!(
                            "Encountered an error sending a message to the DevTools backend: {:?}",
                            e
                        )
                        .into(),
                    ),
                };
            }
            2 | 3 =>
            {
                #[cfg(feature = "web_sys")]
                web_sys::console::error_1(
                    &"Could not open a connection to Yew's developer tools; are they running?"
                        .into(),
                )
            }
            _ => panic!("The WebSocket is in an invalid state."),
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
            std::option_env!("YEW_DEBUGGER_HOST").unwrap_or("localhost"),
            std::option_env!("YEW_DEBUGGER_PORT").unwrap_or("8017".into())
        );
        let return_value = Self {
            ws: match web_sys::WebSocket::new(&ws_url) {
                Ok(s) => s,
                Err(_) => panic!(""),
            },
            message_queue: Vec::new(),
            created_listener: false,
        };
        std::mem::forget(gloo::events::EventListener::new(
            &return_value.ws,
            "close",
            move |_: &web_sys::Event| {
                web_sys::console::error_1(
                    &"Error: the connection to the DevTools backend has closed.".into(),
                );
            },
        ));
        return_value
    }
}

#[cfg(test)]
pub mod tests {
    use super::{DebuggerMessageFlush, DebuggerMessageQueue};
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_message_queuing() {
        crate::dev::DEBUGGER_CONNECTION.with(|debugger| {
            assert_eq!(debugger.borrow_mut().message_queue.len(), 0);
        });
        crate::dev::DEBUGGER_CONNECTION.with(|debugger| {
            debugger.borrow_mut().queue_message("A message");
        });
        crate::dev::DEBUGGER_CONNECTION.with(|debugger| {
            assert_eq!(debugger.borrow().message_queue.len(), 1);
        });
        crate::dev::DEBUGGER_CONNECTION.with(|debugger| {
            std::mem::forget(gloo::events::EventListener::new(
                &debugger.borrow().ws,
                "open",
                |_: &web_sys::Event| {
                    crate::dev::DEBUGGER_CONNECTION.with(|d| {
                        d.borrow_mut().send_messages();
                        assert_eq!(d.borrow().message_queue.len(), 0);
                    })
                },
            ));
            std::mem::forget(gloo::events::EventListener::new(
                &debugger.borrow().ws,
                "close",
                |_: &web_sys::Event| {
                    panic!(
                        "Failed to make a connection: make sure that the test is run with 
                        `YEW_DEBUGGER_HOST` and `YEW_DEBUGGER_PORT` set to the WebSocket echo 
                        server's address.."
                    );
                },
            ));
        });
    }
}
