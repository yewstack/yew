#![recursion_limit = "256"]

use anyhow::Error;
use serde_derive::{Deserialize, Serialize};
use yew::format::{Json, Nothing, Toml};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

type AsBinary = bool;

pub enum Format {
    Json,
    Toml,
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

pub struct Model {
    fetch_service: FetchService,
    ws_service: WebSocketService,
    link: ComponentLink<Model>,
    fetching: bool,
    data: Option<u32>,
    ft: Option<FetchTask>,
    ws: Option<WebSocketTask>,
}

impl Model {
    fn view_data(&self) -> Html {
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

    fn fetch_json(&mut self, binary: AsBinary) -> yew::services::fetch::FetchTask {
        let callback = self.link.callback(
            move |response: Response<Json<Result<DataFromFile, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Msg::FetchReady(data)
                } else {
                    Msg::Ignore // FIXME: Handle this error accordingly.
                }
            },
        );
        let request = Request::get("/data.json").body(Nothing).unwrap();
        if binary {
            self.fetch_service.fetch_binary(request, callback).unwrap()
        } else {
            self.fetch_service.fetch(request, callback).unwrap()
        }
    }

    pub fn fetch_toml(&mut self, binary: AsBinary) -> yew::services::fetch::FetchTask {
        let callback = self.link.callback(
            move |response: Response<Toml<Result<DataFromFile, Error>>>| {
                let (meta, Toml(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Msg::FetchReady(data)
                } else {
                    Msg::Ignore // FIXME: Handle this error accordingly.
                }
            },
        );
        let request = Request::get("/data.toml").body(Nothing).unwrap();
        if binary {
            self.fetch_service.fetch_binary(request, callback).unwrap()
        } else {
            self.fetch_service.fetch(request, callback).unwrap()
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            fetch_service: FetchService::new(),
            ws_service: WebSocketService::new(),
            link,
            fetching: false,
            data: None,
            ft: None,
            ws: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchData(format, binary) => {
                self.fetching = true;
                let task = match format {
                    Format::Json => self.fetch_json(binary),
                    Format::Toml => self.fetch_toml(binary),
                };
                self.ft = Some(task);
            }
            Msg::WsAction(action) => match action {
                WsAction::Connect => {
                    let callback = self.link.callback(|Json(data)| Msg::WsReady(data));
                    let notification = self.link.callback(|status| match status {
                        WebSocketStatus::Opened => Msg::Ignore,
                        WebSocketStatus::Closed | WebSocketStatus::Error => WsAction::Lost.into(),
                    });
                    let task = self
                        .ws_service
                        .connect("ws://localhost:9001/", callback, notification)
                        .unwrap();
                    self.ws = Some(task);
                }
                WsAction::SendData(binary) => {
                    let request = WsRequest { value: 321 };
                    if binary {
                        self.ws.as_mut().unwrap().send_binary(Json(&request));
                    } else {
                        self.ws.as_mut().unwrap().send(Json(&request));
                    }
                }
                WsAction::Disconnect => {
                    self.ws.take();
                }
                WsAction::Lost => {
                    self.ws = None;
                }
            },
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

    fn view(&self) -> Html {
        html! {
            <div>
                <nav class="menu">
                    <button onclick=self.link.callback(|_| Msg::FetchData(Format::Json, false))>
                        { "Fetch Data" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::FetchData(Format::Json, true))>
                        { "Fetch Data [binary]" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::FetchData(Format::Toml, false))>
                        { "Fetch Data [toml]" }
                    </button>
                    { self.view_data() }
                    <button disabled=self.ws.is_some()
                            onclick=self.link.callback(|_| WsAction::Connect)>
                        { "Connect To WebSocket" }
                    </button>
                    <button disabled=self.ws.is_none()
                            onclick=self.link.callback(|_| WsAction::SendData(false))>
                        { "Send To WebSocket" }
                    </button>
                    <button disabled=self.ws.is_none()
                            onclick=self.link.callback(|_| WsAction::SendData(true))>
                        { "Send To WebSocket [binary]" }
                    </button>
                    <button disabled=self.ws.is_none()
                            onclick=self.link.callback(|_| WsAction::Disconnect)>
                        { "Close WebSocket connection" }
                    </button>
                </nav>
            </div>
        }
    }
}
