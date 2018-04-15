extern crate yew;
extern crate dashboard;

use yew::prelude::*;
use yew::services::fetch::FetchService;
use yew::services::websocket::WebSocketService;
use dashboard::Model;

struct Context {
    web: FetchService,
    ws: WebSocketService,
}

impl AsMut<FetchService> for Context {
    fn as_mut(&mut self) -> &mut FetchService {
        &mut self.web
    }
}

impl AsMut<WebSocketService> for Context {
    fn as_mut(&mut self) -> &mut WebSocketService {
        &mut self.ws
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        web: FetchService::new(),
        ws: WebSocketService::new(),
    };
    let app: App<Context, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
