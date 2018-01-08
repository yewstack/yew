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
    gravatar: GravatarService,
    ccxt: CcxtService,
}

struct Model {
    profile: Option<Profile>,
    exchanges: Vec<String>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            profile: None,
            exchanges: Vec::new(),
        }
    }
}

enum Msg {
    Gravatar,
    GravatarReady(Result<Profile, ()>),
    Exchanges,
}

impl Component<Context> for Model {
    type Msg = Msg;

    fn update(&mut self, msg: Msg, context: &mut ScopeRef<Context, Msg>) {
        match msg {
            Msg::Gravatar => {
                let callback = context.send_back(Msg::GravatarReady);
                context.gravatar.profile("205e460b479e2e5b48aec07710c08d50", callback);
            }
            Msg::GravatarReady(Ok(profile)) => {
                self.profile = Some(profile);
            }
            Msg::GravatarReady(Err(_)) => {
                // Can't load gravatar profile
            }
            Msg::Exchanges => {
                self.exchanges = context.ccxt.exchanges();
            }
        }
    }

    fn view(&self) -> Html<Context, Msg> {
        let view_exchange = |exchange| html! {
            <li>{ exchange }</li>
        };
        html! {
            <div>
                <button onclick=|_| Msg::Exchanges,>{ "Get Exchanges" }</button>
                <button onclick=|_| Msg::Gravatar,>{ "Get Gravatar" }</button>
                <ul>
                    { for self.exchanges.iter().map(view_exchange) }
                </ul>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        gravatar: GravatarService::new(),
        ccxt: CcxtService::new(),
    };
    let mut app = Scope::new(context);
    app.mount(Model::default());
    yew::run_loop();
}
