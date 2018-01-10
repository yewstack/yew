//! Service to connect to a servers by
//! [WebSocket Protocol](https://tools.ietf.org/html/rfc6455).

use stdweb::Value;
use format::{Storable, Restorable};
use html::Callback;
use super::Task;

/// A status of a websocket connection. Used for status notification.
pub enum WebSocketStatus {
    /// Fired when a websocket connection was opened.
    Opened,
    /// Fired when a websocket connection was closed.
    Closed,
}

/// A handle to control current websocket connection. Implements `Task` and could be canceled.
pub struct WebSocketHandle(Option<Value>);

/// A websocket service attached to a user context.
pub struct  WebSocketService {
}

impl WebSocketService {
    /// Creates a new service instance connected to `App` by provided `sender`.
    pub fn new() -> Self {
        Self { }
    }

    /// Connects to a server by a weboscket connection. Needs two functions to generate
    /// data and notification messages.
    pub fn connect<OUT: 'static>(&mut self, url: &str, callback: Callback<OUT>, notification: Callback<WebSocketStatus>) -> WebSocketHandle
    where
        OUT: From<Restorable>,
    {
        let callback = move |s: String| {
            let data = Ok(s);
            let out = OUT::from(data);
            callback.emit(out);
        };
        let notify_callback = move |code: u32| {
            let code = {
                match code {
                    1 => WebSocketStatus::Opened,
                    0 => WebSocketStatus::Closed,
                    x => panic!("unknown code of websocket notification: {}", x),
                }
            };
            notification.emit(code);
        };
        let handle = js! {
            var socket = new WebSocket(@{url});
            var callback = @{callback};
            var notify_callback = @{notify_callback};
            socket.onopen = function(event) {
                notify_callback(1);
            };
            socket.onclose = function(event) {
                callback.drop();
                notify_callback(0);
                notify_callback.drop();
            };
            socket.onerror = function(event) {
            };
            socket.onmessage = function(event) {
                callback(event.data);
            };
            return {
                socket,
            };
        };
        WebSocketHandle(Some(handle))
    }
}

impl WebSocketHandle {
    /// Sends data to a websocket connection.
    pub fn send<IN>(&mut self, data: IN)
    where
        IN: Into<Storable>
    {
        if let WebSocketHandle(Some(ref handle)) = *self {
            if let Some(body) = data.into() {
                js! {
                    var handle = @{handle};
                    handle.socket.send(@{body});
                }
            }
        } else {
            panic!("can't send data to the closed websocket connection");
        }
    }
}

impl Task for WebSocketHandle {
    fn cancel(&mut self) {
        let handle = self.0.take().expect("tried to close websocket twice");
        js! {
            var handle = @{handle};
            handle.socket.close();
        }
    }
}
