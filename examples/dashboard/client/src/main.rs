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
    web: FetchService,
    ws: WebSocketService,
}

struct Model {
    fetching: bool,
    data: Option<u32>,
    ws: Option<WebSocketHandle>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            fetching: false,
            data: None,
            ws: None,
        }
    }
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

impl Component<Context> for Model {
    type Msg = Msg;

    fn update(&mut self, msg: Msg, context: &mut ScopeRef<Context, Msg>) {
        match msg {
            Msg::FetchData => {
                self.fetching = true;
                let callback = context.send_back(|Json(data)| Msg::FetchReady(data));
                context.web.fetch(Method::Get, "./data.json", Nothing, callback);
            }
            Msg::WsAction(action) => {
                match action {
                    WsAction::Connect => {
                        let callback = context.send_back(|Json(data)| Msg::WsReady(data));
                        let notification = context.send_back(|status| {
                            match status {
                                WebSocketStatus::Opened => Msg::Ignore,
                                WebSocketStatus::Closed => WsAction::Lost.into(),
                            }
                        });
                        let handle = context.ws.connect("ws://localhost:9001/", callback, notification);
                        self.ws = Some(handle);
                    }
                    WsAction::SendData => {
                        let request = WsRequest {
                            value: 321,
                        };
                        self.ws.as_mut().unwrap().send(Json(&request));
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
            }
        }
    }

    fn view(&self) -> Html<Context, Msg> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_| Msg::FetchData,>{ "Fetch Data" }</button>
                    { self.view_data() }
                    <button disabled=self.ws.is_some(),
                            onclick=|_| WsAction::Connect.into(),>{ "Connect To WebSocket" }</button>
                    <button disabled=self.ws.is_none(),
                            onclick=|_| WsAction::SendData.into(),>{ "Send To WebSocket" }</button>
                    <button disabled=self.ws.is_none(),
                            onclick=|_| WsAction::Disconnect.into(),>{ "Close WebSocket connection" }</button>
                </nav>
            </div>
        }
    }
}

impl Model {
    fn view_data(&self) -> Html<Context, Msg> {
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

fn main() {
    yew::initialize();
    let context = Context {
        web: FetchService::new(),
        ws: WebSocketService::new(),
    };
    let mut app = Scope::new(context);
    app.mount(Model::default());
    yew::run_loop();
}
