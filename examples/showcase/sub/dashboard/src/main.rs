extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

use failure::Error;
use yew::prelude::*;
use yew::format::{Nothing, Json};
use yew::services::Task;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::websocket::{WebSocketService, WebSocketTask, WebSocketStatus};

struct Context {
    web: FetchService,
    ws: WebSocketService,
}

struct Model {
    fetching: bool,
    data: Option<u32>,
    ft: Option<FetchTask>,
    ws: Option<WebSocketTask>,
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
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model {
            fetching: false,
            data: None,
            ft: None,
            ws: None,
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::FetchData => {
                self.fetching = true;
                let callback = context.send_back(|response: Response<Json<Result<DataFromFile, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    println!("META: {:?}, {:?}", meta, data);
                    if meta.status.is_success() {
                        Msg::FetchReady(data)
                    } else {
                        Msg::Ignore  // FIXME: Handle this error accordingly.
                    }
                });
                let request = Request::get("/data.json").body(Nothing).unwrap();
                let task = context.web.fetch(request, callback);
                self.ft = Some(task);
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
                        let task = context.ws.connect("ws://localhost:9001/", callback, notification);
                        self.ws = Some(task);
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
                return false;
            }
        }
        true
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
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
    fn view_data(&self) -> Html<Context, Model> {
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
    let app: App<Context, Model> = App::new(context);
    app.mount_to_body();;
    yew::run_loop();
}
