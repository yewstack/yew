//! A service to connect to a server through the
//! [`WebSocket` Protocol](https://tools.ietf.org/html/rfc6455).

use super::Task;
use cfg_if::cfg_if;
use cfg_match::cfg_match;
use std::fmt;
use yew::callback::Callback;
use yew::format::{Binary, FormatError, Text};
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

/// The status of a WebSocket connection. Used for status notifications.
#[derive(Clone, Debug, PartialEq)]
pub enum WebSocketStatus {
    /// Fired when a WebSocket connection has opened.
    Opened,
    /// Fired when a WebSocket connection has closed.
    Closed,
    /// Fired when a WebSocket connection has failed.
    Error,
}

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
/// An error encountered by a WebSocket.
pub enum WebSocketError {
    #[error("{0}")]
    /// An error encountered when creating the WebSocket.
    CreationError(String),
}

/// A handle to control the WebSocket connection. Implements `Task` and could be canceled.
#[must_use = "the connection will be closed when the task is dropped"]
pub struct WebSocketTask {
    ws: WebSocket,
    notification: Callback<WebSocketStatus>,
    #[cfg(feature = "web_sys")]
    #[allow(dead_code)]
    listeners: [EventListener; 4],
}

#[cfg(feature = "web_sys")]
impl WebSocketTask {
    fn new(
        ws: WebSocket,
        notification: Callback<WebSocketStatus>,
        listener_0: EventListener,
        listeners: [EventListener; 3],
    ) -> Result<WebSocketTask, WebSocketError> {
        let [listener_1, listener_2, listener_3] = listeners;
        Ok(WebSocketTask {
            ws,
            notification,
            listeners: [listener_0, listener_1, listener_2, listener_3],
        })
    }
}

impl fmt::Debug for WebSocketTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WebSocketTask")
    }
}

/// A WebSocket service attached to a user context.
#[derive(Default, Debug)]
pub struct WebSocketService {}

impl WebSocketService {
    /// Connects to a server through a WebSocket connection. Needs two callbacks; one is passed
    /// data, the other is passed updates about the WebSocket's status.
    pub fn connect<OUT: 'static>(
        url: &str,
        callback: Callback<OUT>,
        notification: Callback<WebSocketStatus>,
    ) -> Result<WebSocketTask, WebSocketError>
    where
        OUT: From<Text> + From<Binary>,
    {
        cfg_match! {
            feature = "std_web" => ({
                let ws = Self::connect_common(url, &notification)?.0;
                ws.add_event_listener(move |event: SocketMessageEvent| {
                    process_both(&event, &callback);
                });
                Ok(WebSocketTask { ws, notification })
            }),
            feature = "web_sys" => ({
                let ConnectCommon(ws, listeners) = Self::connect_common(url, &notification)?;
                let listener = EventListener::new(&ws, "message", move |event: &Event| {
                    let event = event.dyn_ref::<MessageEvent>().unwrap();
                    process_both(&event, &callback);
                });
                WebSocketTask::new(ws, notification, listener, listeners)
            }),
        }
    }

    /// Connects to a server through a WebSocket connection, like connect,
    /// but only processes binary frames. Text frames are silently
    /// ignored. Needs two functions to generate data and notification
    /// messages.
    pub fn connect_binary<OUT: 'static>(
        url: &str,
        callback: Callback<OUT>,
        notification: Callback<WebSocketStatus>,
    ) -> Result<WebSocketTask, WebSocketError>
    where
        OUT: From<Binary>,
    {
        cfg_match! {
            feature = "std_web" => ({
                let ws = Self::connect_common(url, &notification)?.0;
                ws.add_event_listener(move |event: SocketMessageEvent| {
                    process_binary(&event, &callback);
                });
                Ok(WebSocketTask { ws, notification })
            }),
            feature = "web_sys" => ({
                let ConnectCommon(ws, listeners) = Self::connect_common(url, &notification)?;
                let listener = EventListener::new(&ws, "message", move |event: &Event| {
                    let event = event.dyn_ref::<MessageEvent>().unwrap();
                    process_binary(&event, &callback);
                });
                WebSocketTask::new(ws, notification, listener, listeners)
            }),
        }
    }

    /// Connects to a server through a WebSocket connection, like connect,
    /// but only processes text frames. Binary frames are silently
    /// ignored. Needs two functions to generate data and notification
    /// messages.
    pub fn connect_text<OUT: 'static>(
        url: &str,
        callback: Callback<OUT>,
        notification: Callback<WebSocketStatus>,
    ) -> Result<WebSocketTask, WebSocketError>
    where
        OUT: From<Text>,
    {
        cfg_match! {
            feature = "std_web" => ({
                let ws = Self::connect_common(url, &notification)?.0;
                ws.add_event_listener(move |event: SocketMessageEvent| {
                    process_text(&event, &callback);
                });
                Ok(WebSocketTask { ws, notification })
            }),
            feature = "web_sys" => ({
                let ConnectCommon(ws, listeners) = Self::connect_common(url, &notification)?;
                let listener = EventListener::new(&ws, "message", move |event: &Event| {
                    let event = event.dyn_ref::<MessageEvent>().unwrap();
                    process_text(&event, &callback);
                });
                WebSocketTask::new(ws, notification, listener, listeners)
            }),
        }
    }

    fn connect_common(
        url: &str,
        notification: &Callback<WebSocketStatus>,
    ) -> Result<ConnectCommon, WebSocketError> {
        let ws = WebSocket::new(url);

        let ws = ws.map_err(
            #[cfg(feature = "std_web")]
            |_| WebSocketError::CreationError("Error opening a WebSocket connection.".to_string()),
            #[cfg(feature = "web_sys")]
            |ws_error| {
                WebSocketError::CreationError(
                    ws_error
                        .unchecked_into::<js_sys::Error>()
                        .to_string()
                        .as_string()
                        .unwrap(),
                )
            },
        )?;

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
        #[cfg_attr(feature = "std_web", allow(clippy::let_unit_value, unused_variables))]
        {
            let listeners = cfg_match! {
                feature = "std_web" => ({
                    ws.add_event_listener(listener_open);
                    ws.add_event_listener(listener_close);
                    ws.add_event_listener(listener_error);
                }),
                feature = "web_sys" => [
                    EventListener::new(&ws, "open", listener_open),
                    EventListener::new(&ws, "close", listener_close),
                    EventListener::new(&ws, "error", listener_error),
                ],
            };
            Ok(ConnectCommon(
                ws,
                #[cfg(feature = "web_sys")]
                listeners,
            ))
        }
    }
}

struct ConnectCommon(WebSocket, #[cfg(feature = "web_sys")] [EventListener; 3]);

fn process_binary<OUT: 'static>(
    #[cfg(feature = "std_web")] event: &SocketMessageEvent,
    #[cfg(feature = "web_sys")] event: &MessageEvent,
    callback: &Callback<OUT>,
) where
    OUT: From<Binary>,
{
    #[cfg(feature = "std_web")]
    let bytes = event.data().into_array_buffer();

    #[cfg(feature = "web_sys")]
    let bytes = if !event.data().is_string() {
        Some(event.data())
    } else {
        None
    };

    let data = if let Some(bytes) = bytes {
        let bytes: Vec<u8> = cfg_match! {
            feature = "std_web" => bytes.into(),
            feature = "web_sys" => Uint8Array::new(&bytes).to_vec(),
        };
        Ok(bytes)
    } else {
        Err(FormatError::ReceivedTextForBinary.into())
    };

    let out = OUT::from(data);
    callback.emit(out);
}

fn process_text<OUT: 'static>(
    #[cfg(feature = "std_web")] event: &SocketMessageEvent,
    #[cfg(feature = "web_sys")] event: &MessageEvent,
    callback: &Callback<OUT>,
) where
    OUT: From<Text>,
{
    let text = cfg_match! {
        feature = "std_web" => event.data().into_text(),
        feature = "web_sys" => event.data().as_string(),
    };

    let data = if let Some(text) = text {
        Ok(text)
    } else {
        Err(FormatError::ReceivedBinaryForText.into())
    };

    let out = OUT::from(data);
    callback.emit(out);
}

fn process_both<OUT: 'static>(
    #[cfg(feature = "std_web")] event: &SocketMessageEvent,
    #[cfg(feature = "web_sys")] event: &MessageEvent,
    callback: &Callback<OUT>,
) where
    OUT: From<Text> + From<Binary>,
{
    #[cfg(feature = "std_web")]
    let is_text = event.data().into_text().is_some();

    #[cfg(feature = "web_sys")]
    let is_text = event.data().is_string();

    if is_text {
        process_text(event, callback);
    } else {
        process_binary(event, callback);
    }
}

impl WebSocketTask {
    /// Sends data to a WebSocket connection.
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

    /// Sends binary data to a WebSocket connection.
    pub fn send_binary<IN>(&mut self, data: IN)
    where
        IN: Into<Binary>,
    {
        if let Ok(body) = data.into() {
            let result = cfg_match! {
                feature = "std_web" => self.ws.send_bytes(&body),
                feature = "web_sys" => self.ws.send_with_u8_array(&body),
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
}

impl Drop for WebSocketTask {
    fn drop(&mut self) {
        if self.is_active() {
            cfg_match! {
                feature = "std_web" => self.ws.close(),
                feature = "web_sys" => self.ws.close().ok(),
            };
        }
    }
}

#[cfg(test)]
#[cfg(all(feature = "wasm_test", feature = "echo_server_test"))]
mod tests {
    use super::*;
    use crate::callback_test_util::CallbackFuture;
    use crate::TimeoutService;
    use serde::{Deserialize, Serialize};
    use std::time::Duration;
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
    use yew::callback::Callback;
    use yew::format::{FormatError, Json};

    wasm_bindgen_test_configure!(run_in_browser);

    const fn echo_server_url() -> &'static str {
        // we can't do this at runtime because we're running in the browser.
        env!("ECHO_SERVER_URL")
    }

    // Ignore the first response from the echo server
    async fn ignore_first_message<T>(cb_future: &CallbackFuture<T>) {
        let sleep_future = CallbackFuture::<()>::default();
        let _sleep_task =
            TimeoutService::spawn(Duration::from_millis(10), sleep_future.clone().into());
        sleep_future.await;
        cb_future.ready();
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Message {
        test: String,
    }

    #[test]
    async fn connect() {
        let url = echo_server_url();
        let cb_future = CallbackFuture::<Json<Result<Message, anyhow::Error>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let status_future = CallbackFuture::<WebSocketStatus>::default();
        let notification: Callback<_> = status_future.clone().into();

        let mut task = WebSocketService::connect(url, callback, notification).unwrap();
        assert_eq!(status_future.await, WebSocketStatus::Opened);
        ignore_first_message(&cb_future).await;

        let msg = Message {
            test: String::from("hello"),
        };

        task.send(Json(&msg));
        match cb_future.clone().await {
            Json(Ok(received)) => assert_eq!(received, msg),
            Json(Err(err)) => assert!(false, err),
        }

        task.send_binary(Json(&msg));
        match cb_future.await {
            Json(Ok(received)) => assert_eq!(received, msg),
            Json(Err(err)) => assert!(false, err),
        }
    }

    #[test]
    #[cfg(feature = "web_sys")]
    async fn test_invalid_url_error() {
        let url = "syntactically-invalid";
        let cb_future = CallbackFuture::<Json<Result<Message, anyhow::Error>>>::default();
        let callback = cb_future.clone().into();
        let status_future = CallbackFuture::<WebSocketStatus>::default();
        let notification: Callback<_> = status_future.clone().into();
        let task = WebSocketService::connect_text(url, callback, notification);
        assert!(task.is_err());
        if let Err(err) = task {
            #[allow(irrefutable_let_patterns)]
            if let WebSocketError::CreationError(creation_err) = err {
                assert!(creation_err.starts_with("SyntaxError:"));
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    async fn connect_text() {
        let url = echo_server_url();
        let cb_future = CallbackFuture::<Json<Result<Message, anyhow::Error>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let status_future = CallbackFuture::<WebSocketStatus>::default();
        let notification: Callback<_> = status_future.clone().into();

        let mut task = WebSocketService::connect_text(url, callback, notification).unwrap();
        assert_eq!(status_future.await, WebSocketStatus::Opened);
        ignore_first_message(&cb_future).await;

        let msg = Message {
            test: String::from("hello"),
        };

        task.send(Json(&msg));
        match cb_future.clone().await {
            Json(Ok(received)) => assert_eq!(received, msg),
            Json(Err(err)) => assert!(false, err),
        }

        task.send_binary(Json(&msg));
        match cb_future.await {
            Json(Ok(received)) => assert!(false, received),
            Json(Err(err)) => assert_eq!(
                err.to_string(),
                FormatError::ReceivedBinaryForText.to_string()
            ),
        }
    }

    #[test]
    async fn connect_binary() {
        let url = echo_server_url();
        let cb_future = CallbackFuture::<Json<Result<Message, anyhow::Error>>>::default();
        let callback: Callback<_> = cb_future.clone().into();
        let status_future = CallbackFuture::<WebSocketStatus>::default();
        let notification: Callback<_> = status_future.clone().into();

        let mut task = WebSocketService::connect_binary(url, callback, notification).unwrap();
        assert_eq!(status_future.await, WebSocketStatus::Opened);
        ignore_first_message(&cb_future).await;

        let msg = Message {
            test: String::from("hello"),
        };

        task.send_binary(Json(&msg));
        match cb_future.clone().await {
            Json(Ok(received)) => assert_eq!(received, msg),
            Json(Err(err)) => assert!(false, err),
        }

        task.send(Json(&msg));
        match cb_future.await {
            Json(Ok(received)) => assert!(false, received),
            Json(Err(err)) => assert_eq!(
                err.to_string(),
                FormatError::ReceivedTextForBinary.to_string()
            ),
        }
    }
}
