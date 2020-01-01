//! Service to connect to a servers by
//! [`WebSocket` Protocol](https://tools.ietf.org/html/rfc6455).

use super::Task;
use crate::callback::Callback;
use crate::format::{Binary, Text};
use std::fmt;
#[cfg(feature = "std_web")]
use stdweb::{
    traits::IMessageEvent,
    web::{
        event::{SocketCloseEvent, SocketErrorEvent, SocketMessageEvent, SocketOpenEvent},
        IEventTarget, SocketBinaryType, SocketReadyState, WebSocket,
    },
};
#[cfg(feature = "web_sys")]
use ::{
    gloo::events::EventListener,
    js_sys::Uint8Array,
    wasm_bindgen::JsCast,
    web_sys::{BinaryType, Event, MessageEvent, WebSocket},
};

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
    #[cfg(feature = "web_sys")]
    #[allow(dead_code)]
    listeners: [EventListener; 4],
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
    ) -> Result<WebSocketTask, &str>
    where
        OUT: From<Text> + From<Binary>,
    {
        let ws = WebSocket::new(url);
        if ws.is_err() {
            return Err("Failed to created websocket with given URL");
        }

        let ws = ws.unwrap();
        #[cfg(feature = "std_web")]
        ws.set_binary_type(SocketBinaryType::ArrayBuffer);
        #[cfg(feature = "web_sys")]
        ws.set_binary_type(BinaryType::Arraybuffer);
        let notify = notification.clone();
        let listener = move |#[cfg(feature = "std_web")] _: SocketOpenEvent,
                             #[cfg(feature = "web_sys")] _: &Event| {
            notify.emit(WebSocketStatus::Opened);
        };
        #[cfg(feature = "std_web")]
        ws.add_event_listener(listener);
        #[cfg(feature = "web_sys")]
        let listener_open = EventListener::new(&ws, "open", listener);
        let notify = notification.clone();
        let listener = move |#[cfg(feature = "std_web")] _: SocketCloseEvent,
                             #[cfg(feature = "web_sys")] _: &Event| {
            notify.emit(WebSocketStatus::Closed);
        };
        #[cfg(feature = "std_web")]
        ws.add_event_listener(listener);
        #[cfg(feature = "web_sys")]
        let listener_close = EventListener::new(&ws, "close", listener);
        let notify = notification.clone();
        let listener = move |#[cfg(feature = "std_web")] _: SocketErrorEvent,
                             #[cfg(feature = "web_sys")] _: &Event| {
            notify.emit(WebSocketStatus::Error);
        };
        #[cfg(feature = "std_web")]
        ws.add_event_listener(listener);
        #[cfg(feature = "web_sys")]
        let listener_error = EventListener::new(&ws, "error", listener);
        let listener = move |#[cfg(feature = "std_web")] event: SocketMessageEvent,
                             #[cfg(feature = "web_sys")] event: &Event| {
            #[cfg(feature = "web_sys")]
            let data = event.dyn_ref::<MessageEvent>().unwrap().data();
            #[cfg(feature = "std_web")]
            let text = event.data().into_text();
            #[cfg(feature = "web_sys")]
            let text = data.as_string();
            #[cfg(feature = "std_web")]
            let bytes = event.data().into_array_buffer();
            #[cfg(feature = "web_sys")]
            let bytes = Some(data);

            if let Some(text) = text {
                let data = Ok(text);
                let out = OUT::from(data);
                callback.emit(out);
            } else if let Some(bytes) = bytes {
                #[cfg(feature = "std_web")]
                let bytes: Vec<u8> = bytes.into();
                #[cfg(feature = "web_sys")]
                let bytes = Uint8Array::new_with_byte_offset(&bytes, 0).to_vec();
                let data = Ok(bytes);
                let out = OUT::from(data);
                callback.emit(out);
            }
        };
        #[cfg(feature = "std_web")]
        ws.add_event_listener(listener);
        #[cfg(feature = "web_sys")]
        let listener_message = EventListener::new(&ws, "message", listener);
        Ok(WebSocketTask {
            ws,
            notification,
            #[cfg(feature = "web_sys")]
            listeners: [
                listener_open,
                listener_close,
                listener_error,
                listener_message,
            ],
        })
    }
}

impl WebSocketTask {
    /// Sends data to a websocket connection.
    pub fn send<IN>(&mut self, data: IN)
    where
        IN: Into<Text>,
    {
        if let Ok(body) = data.into() {
            #[cfg(feature = "std_web")]
            let result = self.ws.send_text(&body);
            #[cfg(feature = "web_sys")]
            let result = self.ws.send_with_str(&body);

            if result.is_err() {
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
            #[cfg(feature = "std_web")]
            let result = self.ws.send_bytes(&body);
            #[cfg(feature = "web_sys")]
            let result = {
                let mut body = body;
                self.ws.send_with_u8_array(&mut body)
            };

            if result.is_err() {
                self.notification.emit(WebSocketStatus::Error);
            }
        }
    }
}

impl Task for WebSocketTask {
    fn is_active(&self) -> bool {
        #[cfg(feature = "std_web")]
        {
            self.ws.ready_state() == SocketReadyState::Open
        }
        #[cfg(feature = "web_sys")]
        {
            self.ws.ready_state() == WebSocket::OPEN
        }
    }
    fn cancel(&mut self) {
        #[cfg(feature = "std_web")]
        self.ws.close();
        #[cfg(feature = "web_sys")]
        self.ws.close().unwrap();
    }
}

impl Drop for WebSocketTask {
    fn drop(&mut self) {
        if self.is_active() {
            self.cancel();
        }
    }
}
