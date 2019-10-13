//! Service to connect to a servers by
//! [`WebSocket` Protocol](https://tools.ietf.org/html/rfc6455).

use super::Task;
use crate::callback::Callback;
use crate::format::{Binary, Text};
use std::fmt;
use stdweb::traits::IMessageEvent;
use stdweb::web::event::{SocketCloseEvent, SocketErrorEvent, SocketMessageEvent, SocketOpenEvent};
use stdweb::web::{IEventTarget, SocketBinaryType, SocketReadyState, WebSocket};

/// A status of a websocket connection. Used for status notification.
#[derive(Debug)]
pub enum WebSocketStatus {
    /// Fired when a websocket connection was opened.
    Opened,
    /// Fired when a websocket connection was closed.
    Closed,
    /// Fired when a websocket connection was failed.
    Error,
}

/// A handle to control current websocket connection. Implements `Task` and could be canceled.
#[must_use]
pub struct WebSocketTask {
    ws: WebSocket,
    notification: Callback<WebSocketStatus>,
}

impl fmt::Debug for WebSocketTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WebSocketTask")
    }
}

/// A websocket service attached to a user context.
#[derive(Default, Debug)]
pub struct WebSocketService {}

impl WebSocketService {
    /// Creates a new service instance connected to `App` by provided `sender`.
    pub fn new() -> Self {
        Self {}
    }

    /// Connects to a server by a websocket connection. Needs two functions to generate
    /// data and notification messages.
    pub fn connect<OUT: 'static>(
        &mut self,
        url: &str,
        callback: Callback<OUT>,
        notification: Callback<WebSocketStatus>,
    ) -> WebSocketTask
    where
        OUT: From<Text> + From<Binary>,
    {
        let ws = WebSocket::new(url).unwrap();
        ws.set_binary_type(SocketBinaryType::ArrayBuffer);
        let notify = notification.clone();
        ws.add_event_listener(move |_: SocketOpenEvent| {
            notify.emit(WebSocketStatus::Opened);
        });
        let notify = notification.clone();
        ws.add_event_listener(move |_: SocketCloseEvent| {
            notify.emit(WebSocketStatus::Closed);
        });
        let notify = notification.clone();
        ws.add_event_listener(move |_: SocketErrorEvent| {
            notify.emit(WebSocketStatus::Error);
        });
        ws.add_event_listener(move |event: SocketMessageEvent| {
            if let Some(bytes) = event.data().into_array_buffer() {
                let bytes: Vec<u8> = bytes.into();
                let data = Ok(bytes);
                let out = OUT::from(data);
                callback.emit(out);
            } else if let Some(text) = event.data().into_text() {
                let data = Ok(text);
                let out = OUT::from(data);
                callback.emit(out);
            }
        });
        WebSocketTask { ws, notification }
    }
}

impl WebSocketTask {
    /// Sends data to a websocket connection.
    pub fn send<IN>(&mut self, data: IN)
    where
        IN: Into<Text>,
    {
        if let Ok(body) = data.into() {
            if self.ws.send_text(&body).is_err() {
                self.notification.emit(WebSocketStatus::Error);
            }
        }
    }

    /// Sends binary data to a websocket connection.
    pub fn send_binary<IN>(&mut self, data: IN)
    where
        IN: Into<Binary>,
    {
        if let Ok(body) = data.into() {
            if self.ws.send_bytes(&body).is_err() {
                self.notification.emit(WebSocketStatus::Error);
            }
        }
    }
}

impl Task for WebSocketTask {
    fn is_active(&self) -> bool {
        self.ws.ready_state() == SocketReadyState::Open
    }
    fn cancel(&mut self) {
        self.ws.close();
    }
}

impl Drop for WebSocketTask {
    fn drop(&mut self) {
        if self.is_active() {
            self.cancel();
        }
    }
}
