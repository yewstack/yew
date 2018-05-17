#![recursion_limit = "128"]

extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

use failure::Error;
use yew::prelude::*;
use yew::format::{Nothing, Json, Toml};
use yew::services::Task;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::websocket::{WebSocketService, WebSocketTask, WebSocketStatus};

type AsBinary = bool;

pub enum Format {
    Json,
    Toml,
}

pub struct Model {
    fetching: bool,
    data: Option<u32>,
    ft: Option<FetchTask>,
    ws: Option<WebSocketTask>,
}

pub enum WsAction {
    Connect,
    SendData(AsBinary),
    Disconnect,
    Lost,
}

pub enum Msg {
    FetchData(Format, AsBinary),
    WsAction(WsAction),
    FetchReady(Result<DataFromFile, Error>),
    WsReady(Result<WsResponse, Error>),
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
pub struct DataFromFile {
    value: u32,
}

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Debug)]
struct WsRequest {
    value: u32,
}

/// This type is an expected response from a websocket connection.
#[derive(Deserialize, Debug)]
pub struct WsResponse {
    value: u32,
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<FetchService> + AsMut<WebSocketService> + 'static,
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {
            fetching: false,
            data: None,
            ft: None,
            ws: None,
        }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::FetchData(format, binary) => {
                self.fetching = true;
                let task = match format {
                    Format::Json => {
                        let callback = env.send_back(move |response: Response<Json<Result<DataFromFile, Error>>>| {
                            let (meta, Json(data)) = response.into_parts();
                            println!("META: {:?}, {:?}", meta, data);
                            if meta.status.is_success() {
                                Msg::FetchReady(data)
                            } else {
                                Msg::Ignore  // FIXME: Handle this error accordingly.
                            }
                        });
                        let request = Request::get("/data.json").body(Nothing).unwrap();
                        let fetch_service: &mut FetchService = env.as_mut();
                        if binary {
                            fetch_service.fetch_binary(request, callback)
                        } else {
                            fetch_service.fetch(request, callback)
                        }
                    },
                    Format::Toml => {
                        let callback = env.send_back(move |response: Response<Toml<Result<DataFromFile, Error>>>| {
                            let (meta, Toml(data)) = response.into_parts();
                            println!("META: {:?}, {:?}", meta, data);
                            if meta.status.is_success() {
                                Msg::FetchReady(data)
                            } else {
                                Msg::Ignore  // FIXME: Handle this error accordingly.
                            }
                        });
                        let request = Request::get("/data.toml").body(Nothing).unwrap();
                        let fetch_service: &mut FetchService = env.as_mut();
                        if binary {
                            fetch_service.fetch_binary(request, callback)
                        } else {
                            fetch_service.fetch(request, callback)
                        }
                    },
                };
                self.ft = Some(task);
            }
            Msg::WsAction(action) => {
                match action {
                    WsAction::Connect => {
                        let callback = env.send_back(|Json(data)| Msg::WsReady(data));
                        let notification = env.send_back(|status| {
                            match status {
                                WebSocketStatus::Opened => Msg::Ignore,
                                WebSocketStatus::Closed | WebSocketStatus::Error => WsAction::Lost.into(),
                            }
                        });
                        let ws_service: &mut WebSocketService = env.as_mut();
                        let task = ws_service.connect("ws://localhost:9001/", callback, notification);
                        self.ws = Some(task);
                    }
                    WsAction::SendData(binary) => {
                        let request = WsRequest {
                            value: 321,
                        };
                        if binary {
                            self.ws.as_mut().unwrap().send_binary(Json(&request));
                        } else {
                            self.ws.as_mut().unwrap().send(Json(&request));
                        }
                    }
                    WsAction::Disconnect => {
                        self.ws.take().unwrap().cancel();
                    }
                    WsAction::Lost => {
                        self.ws = None;
                    }
                }
            }
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.data = response.map(|data| data.value).ok();
            }
            Msg::WsReady(response) => {
                self.data = response.map(|data| data.value).ok();
            }
            Msg::Ignore => {
                return false;
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<FetchService> + AsMut<WebSocketService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_| Msg::FetchData(Format::Json, false),>{ "Fetch Data" }</button>
                    <button onclick=|_| Msg::FetchData(Format::Json, true),>{ "Fetch Data [binary]" }</button>
                    <button onclick=|_| Msg::FetchData(Format::Toml, false),>{ "Fetch Data [toml]" }</button>
                    { self.view_data() }
                    <button disabled=self.ws.is_some(),
                            onclick=|_| WsAction::Connect.into(),>{ "Connect To WebSocket" }</button>
                    <button disabled=self.ws.is_none(),
                            onclick=|_| WsAction::SendData(false).into(),>{ "Send To WebSocket" }</button>
                    <button disabled=self.ws.is_none(),
                            onclick=|_| WsAction::SendData(true).into(),>{ "Send To WebSocket [binary]" }</button>
                    <button disabled=self.ws.is_none(),
                            onclick=|_| WsAction::Disconnect.into(),>{ "Close WebSocket connection" }</button>
                </nav>
            </div>
        }
    }

}

impl Model {
    fn view_data<CTX>(&self) -> Html<CTX, Model>
    where
        CTX: AsMut<FetchService> + AsMut<WebSocketService> + 'static,
    {
        if let Some(value) = self.data {
            html! {
                <p>{ value }</p>
            }
        } else {
            html! {
                <p>{ "Data hasn't fetched yet." }</p>
            }
        }
    }
}
