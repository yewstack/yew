#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

use yew::html::*;
use yew::format::{Nothing, Json};
use yew::services::Task;
use yew::services::fetch::{FetchService, Method};
use yew::services::websocket::{WebSocketService, WebSocketHandle, WebSocketStatus};

struct Context {
    web: FetchService<Msg>,
    ws: WebSocketService<Msg>,
}

struct Model {
    fetching: bool,
    data: Option<u32>,
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
    FetchReady(Result<DataFromFile, ()>),
    WsReady(Result<WsResponse, ()>),
    Ignore,
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Msg::WsAction(action)
    }
}

/// This type is used to parse data from `./static/data.json` file and
/// have to correspond the data layout from that file.
#[derive(Deserialize, Debug)]
struct DataFromFile {
    value: u32,
}

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Debug)]
struct WsRequest {
    value: u32,
}

/// This type is an expected response from a websocket connection.
#[derive(Deserialize, Debug)]
struct WsResponse {
    value: u32,
}

fn ws_status_to_msg(status: WebSocketStatus) -> Msg {
    match status {
        WebSocketStatus::Opened => Msg::Ignore,
        WebSocketStatus::Closed => WsAction::Lost.into(),
    }
}

fn update(context: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::FetchData => {
            model.fetching = true;
            context.web.fetch(Method::Get, "./data.json", Nothing, |Json(data)| Msg::FetchReady(data));
        }
        Msg::WsAction(action) => {
            match action {
                WsAction::Connect => {
                    let handle = context.ws.connect(
                        "ws://localhost:9001/",
                        |Json(data)| Msg::WsReady(data),
                        ws_status_to_msg
                    );
                    model.ws = Some(handle);
                }
                WsAction::SendData => {
                    let request = WsRequest {
                        value: 321,
                    };
                    model.ws.as_mut().unwrap().send(Json(&request));
                }
                WsAction::Disconnect => {
                    model.ws.take().unwrap().cancel();
                }
                WsAction::Lost => {
                    model.ws = None;
                }
            }
        }
        Msg::FetchReady(response) => {
            model.fetching = false;
            model.data = response.map(|data| data.value).ok();
        }
        Msg::WsReady(response) => {
            model.data = response.map(|data| data.value).ok();
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
    if let Some(value) = model.data {
        html! {
            <p>{ value }</p>
        }
    } else {
        html! {
            <p>{ "Data hasn't fetched yet." }</p>
        }
    }
}

fn main() {
    let mut app = App::new();
    let context = Context {
        web: FetchService::new(app.sender()),
        ws: WebSocketService::new(app.sender()),
    };
    let model = Model {
        fetching: false,
        data: None,
        ws: None,
    };
    app.run(context, model, update, view);
}
