//! Developer tools.
//! These communicate with a backend over a WebSocket connection.
//! Messages are sent as JSON.

use serde::Serialize;
use serde_json;
use web_sys;

pub mod messages;

#[cfg(feature = "std_web")]
panic!("The `dev` feature currently only works with web-sys.");

/// Stores a connection to the DevTools server.
#[derive(Debug)]
pub struct DebuggerConnection {
    ws: web_sys::WebSocket,
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
    /// Creates a new connection to `localhost` on the default port 8017.
    pub fn new() -> Self {
        Self {
            ws: web_sys::WebSocket::new("localhost:8017").unwrap(),
        }
    }
}

impl<T: Serialize> Debugger<T> for DebuggerConnection {
    fn send_message(&self, message: T) {
        match self
            .ws
            .send_with_str(&serde_json::to_string(&message).unwrap())
        {
            Ok(_) => {}
            Err(e) => println!("Error sending debug message: {:?}", e),
        };
    }
}

#[cfg(test)]
pub mod tests {}
