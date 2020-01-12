//! Service to connect to a servers by
//! [`WebSocket` Protocol](https://tools.ietf.org/html/rfc6455).

use super::Task;
use crate::callback::Callback;
use crate::format::{Binary, Text};
use cfg_if::cfg_if;
use cfg_match::cfg_match;
use std::fmt;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::traits::IMessageEvent;
        use stdweb::web::event::{SocketCloseEvent, SocketErrorEvent, SocketMessageEvent, SocketOpenEvent};
        use stdweb::web::{IEventTarget, SocketBinaryType, SocketReadyState, WebSocket};
    } else if #[cfg(feature = "web_sys")] {
        use gloo::events::EventListener;
        use js_sys::Uint8Array;
        use wasm_bindgen::JsCast;
        use web_sys::{BinaryType, Event, MessageEvent, WebSocket};
    }
}

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
    listeners: Option<[EventListener; 4]>,
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

        let ws = ws.map_err(|_| "failed to build websocket")?;
        cfg_match! {
            feature = "std_web" => ws.set_binary_type(SocketBinaryType::ArrayBuffer),
            feature = "web_sys" => ws.set_binary_type(BinaryType::Arraybuffer),
        };
        let notify = notification.clone();
        let listener_open =
            move |#[cfg(feature = "std_web")] _: SocketOpenEvent,
                  #[cfg(feature = "web_sys")] _: &Event| {
                notify.emit(WebSocketStatus::Opened);
            };
        let notify = notification.clone();
        let listener_close =
            move |#[cfg(feature = "std_web")] _: SocketCloseEvent,
                  #[cfg(feature = "web_sys")] _: &Event| {
                notify.emit(WebSocketStatus::Closed);
            };
        let notify = notification.clone();
        let listener_error =
            move |#[cfg(feature = "std_web")] _: SocketErrorEvent,
                  #[cfg(feature = "web_sys")] _: &Event| {
                notify.emit(WebSocketStatus::Error);
            };
        let listener_message =
            move |#[cfg(feature = "std_web")] event: SocketMessageEvent,
                  #[cfg(feature = "web_sys")] event: &Event| {
                #[cfg(feature = "web_sys")]
                let data = event.dyn_ref::<MessageEvent>().unwrap().data();
                let text = cfg_match! {
                    feature = "std_web" => event.data().into_text(),
                    feature = "web_sys" => data.as_string(),
                };
                let bytes = cfg_match! {
                    feature = "std_web" => event.data().into_array_buffer(),
                    feature = "web_sys" => Some(data),
                };

                if let Some(text) = text {
                    let data = Ok(text);
                    let out = OUT::from(data);
                    callback.emit(out);
                } else if let Some(bytes) = bytes {
                    let bytes: Vec<u8> = cfg_match! {
                        feature = "std_web" => bytes.into(),
                        feature = "web_sys" => Uint8Array::new_with_byte_offset(&bytes, 0).to_vec(),
                    };
                    let data = Ok(bytes);
                    let out = OUT::from(data);
                    callback.emit(out);
                }
            };
        #[cfg_attr(feature = "std_web", allow(clippy::let_unit_value, unused_variables))]
        {
            let listeners = cfg_match! {
                feature = "std_web" => ({
                    ws.add_event_listener(listener_open);
                    ws.add_event_listener(listener_close);
                    ws.add_event_listener(listener_error);
                    ws.add_event_listener(listener_message);
                }),
                feature = "web_sys" => ({
                    Some([
                        EventListener::new(&ws, "open", listener_open),
                        EventListener::new(&ws, "close", listener_close),
                        EventListener::new(&ws, "error", listener_error),
                        EventListener::new(&ws, "message", listener_message),
                    ])
                }),
            };
            Ok(WebSocketTask {
                ws,
                notification,
                #[cfg(feature = "web_sys")]
                listeners,
            })
        }
    }
}

impl WebSocketTask {
    /// Sends data to a websocket connection.
    pub fn send<IN>(&mut self, data: IN)
    where
        IN: Into<Text>,
    {
        if let Ok(body) = data.into() {
            let result = cfg_match! {
                feature = "std_web" => self.ws.send_text(&body),
                feature = "web_sys" => self.ws.send_with_str(&body),
            };

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
            let result = cfg_match! {
                feature = "std_web" => self.ws.send_bytes(&body),
                feature = "web_sys" => ({
                    let mut body = body;
                    self.ws.send_with_u8_array(&mut body)
                }),
            };

            if result.is_err() {
                self.notification.emit(WebSocketStatus::Error);
            }
        }
    }
}

impl Task for WebSocketTask {
    fn is_active(&self) -> bool {
        cfg_match! {
            feature = "std_web" => self.ws.ready_state() == SocketReadyState::Open,
            feature = "web_sys" => self.ws.ready_state() == WebSocket::OPEN,
        }
    }
    fn cancel(&mut self) {
        cfg_match! {
            feature = "std_web" => self.ws.close(),
            feature = "web_sys" => ({
                self.ws.close().unwrap();
                drop(self.listeners.take().expect("tried to cancel websocket twice"));
            }),
        };
    }
}

impl Drop for WebSocketTask {
    fn drop(&mut self) {
        if self.is_active() {
            self.cancel();
        }
    }
}
