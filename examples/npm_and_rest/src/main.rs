#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::*;

// Own services implementation
mod gravatar;
use gravatar::{GravatarService, Profile};
mod ccxt;
use ccxt::CcxtService;

struct Context {
    gravatar: GravatarService,
    ccxt: CcxtService,
}

struct Model {
    profile: Option<Profile>,
    exchanges: Vec<String>,
}

enum Msg {
    Gravatar,
    GravatarReady(Result<Profile, ()>),
    Exchanges,
}

fn update(context: &mut AppContext<Context, Model, Msg>, model: &mut Model, msg: Msg) -> ShouldRender {
    match msg {
        Msg::Gravatar => {
            let callback = context.send_back(Msg::GravatarReady);
            context.gravatar.profile("205e460b479e2e5b48aec07710c08d50", callback);
        }
        Msg::GravatarReady(Ok(profile)) => {
            model.profile = Some(profile);
        }
        Msg::GravatarReady(Err(_)) => {
            // Can't load gravatar profile
        }
        Msg::Exchanges => {
            model.exchanges = context.ccxt.exchanges();
        }
    }
    true
}

fn view(model: &Model) -> AppHtml<Context, Model, Msg> {
    let view_exchange = |exchange| html! {
        <li>{ exchange }</li>
    };
    html! {
        <div>
            <button onclick=|_| Msg::Exchanges,>{ "Get Exchanges" }</button>
            <button onclick=|_| Msg::Gravatar,>{ "Get Gravatar" }</button>
            <ul>
                { for model.exchanges.iter().map(view_exchange) }
            </ul>
        </div>
    }
}

fn main() {
    yew::initialize();
    let app = App::new();
    let context = Context {
        gravatar: GravatarService::new(),
        ccxt: CcxtService::new(),
    };
    let model = Model {
        profile: None,
        exchanges: Vec::new(),
    };
    app.mount(context, model, update, view);
    yew::run_loop();
}
