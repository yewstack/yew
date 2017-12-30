use stdweb::Value;
use html::Context;
use services::format::{Storable, Restorable};
use super::Task;

pub struct WebSocketHandle(Option<Value>);

pub trait WebSocketService<MSG> {
    fn ws_connect<F, OUT>(&mut self, url: &str, converter: F) -> WebSocketHandle
    where
        OUT: From<Restorable>,
        F: Fn(OUT) -> MSG + 'static;
}

impl<MSG: 'static> WebSocketService<MSG> for Context<MSG> {
    fn ws_connect<F, OUT>(&mut self, url: &str, converter: F) -> WebSocketHandle
    where
        OUT: From<Restorable>,
        F: Fn(OUT) -> MSG + 'static
    {
        let mut tx = self.sender();
        let callback = move |s: String| {
            let data = Ok(s);
            let out = OUT::from(data);
            let msg = converter(out);
            tx.send(msg);
        };
        let handle = js! {
            var socket = new WebSocket(@{url});
            var callback = @{callback};
            socket.addEventListener("message", function (event) {
                callback(event.data);
            });
            return {
                socket,
                callback,
            };
        };
        WebSocketHandle(Some(handle))
    }
}

impl WebSocketHandle {
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
            handle.callback.drop();
        }
    }
}
