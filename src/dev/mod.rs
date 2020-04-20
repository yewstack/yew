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
    /// Creates a new connection to `localhost` on the default port 8017.
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "web_sys")]
            ws: web_sys::WebSocket::new("localhost:8017").unwrap(),
            #[cfg(feature = "std_web")]
            ws: match stdweb::web::WebSocket::new("localhost:8017") {
                Ok(s) => s,
                Err(_) => {
                    stdweb::console!(error, "Error: could not connect to the WebSocket. Are you sure the DevTools backend is running?");
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
pub mod tests {}
