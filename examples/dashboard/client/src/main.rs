#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

use yew::html::*;
use yew::services::Task;
use yew::services::format::{Nothing, Json};
use yew::services::fetch::{FetchService, Method};
use yew::services::websocket::{WebSocketService, WebSocketHandle, WebSocketStatus};

struct Model {
    fetching: bool,
    data: Option<Status>,
    ws: Option<WebSocketHandle>,
}

enum WsAction {
    Connect,
    SendData,
    Disconnect,
    Lost,
}

enum Msg {
    FetchData,
    WsAction(WsAction),
    DataReady(Result<Status, ()>),
    Ignore,
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Msg::WsAction(action)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Status {
    value: u32,
}

fn ws_status_to_msg(status: WebSocketStatus) -> Msg {
    match status {
        WebSocketStatus::Opened => Msg::Ignore,
        WebSocketStatus::Closed => WsAction::Lost.into(),
    }
}

fn update(context: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    match msg {
        Msg::FetchData => {
            context.fetch(Method::Get, "./data.json", Nothing, |Json(data)| Msg::DataReady(data));
        }
        Msg::WsAction(action) => {
            match action {
                WsAction::Connect => {
                    let handle = context.ws_connect(
                        "ws://localhost:9001/",
                        |Json(data)| Msg::DataReady(data),
                        ws_status_to_msg
                    );
                    model.ws = Some(handle);
                }
                WsAction::SendData => {
                    let data = Status {
                        value: 321,
                    };
                    model.ws.as_mut().unwrap().send(Json(&data));
                }
                WsAction::Disconnect => {
                    model.ws.take().unwrap().cancel();
                }
                WsAction::Lost => {
                    model.ws = None;
                }
            }
        }
        Msg::DataReady(response) => {
            model.fetching = false;
            match response {
                Ok(data) => {
                    model.data = Some(data);
                }
                Err(_) => {
                }
            }
        }
        Msg::Ignore => {
        }
    }
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            <nav class="menu",>
                <button onclick=|_| Msg::FetchData,>{ "Fetch Data" }</button>
                { view_data(model) }
                <button disabled=model.ws.is_some(),
                        onclick=|_| WsAction::Connect.into(),>{ "Connect To WebSocket" }</button>
                <button disabled=model.ws.is_none(),
                        onclick=|_| WsAction::SendData.into(),>{ "Send To WebSocket" }</button>
                <button disabled=model.ws.is_none(),
                        onclick=|_| WsAction::Disconnect.into(),>{ "Close WebSocket connection" }</button>
            </nav>
        </div>
    }
}

fn view_data(model: &Model) -> Html<Msg> {
    if let Some(ref data) = model.data {
        html! {
            <p>{ data.value }</p>
        }
    } else {
        html! {
            <p>{ "Data hasn't fetched yet." }</p>
        }
    }
}

fn main() {
    let model = Model {
        fetching: false,
        data: None,
        ws: None,
    };
    program(model, update, view);
}
