#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::html::*;

// Own services implementation
mod gravatar;
use gravatar::{GravatarService, Profile};
mod ccxt;
use ccxt::CcxtService;

struct Context {
    gravatar: GravatarService<Msg>,
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

fn update(context: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Gravatar => {
            context.gravatar.profile("205e460b479e2e5b48aec07710c08d50", Msg::GravatarReady);
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
}

fn view(model: &Model) -> Html<Msg> {
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
    let mut app = App::new();
    let context = Context {
        gravatar: GravatarService::new(app.sender()),
        ccxt: CcxtService::new(),
    };
    let model = Model {
        profile: None,
        exchanges: Vec::new(),
    };
    app.mount(context, model, update, view);
    yew::run_loop();
}
