#![recursion_limit = "128"]

#[macro_use]
extern crate stdweb;

// Own services implementation
pub mod gravatar;
pub mod ccxt;

use failure::Error;
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::fetch::FetchTask;

use gravatar::{GravatarService, Profile};
use ccxt::CcxtService;

pub struct Model {
    gravatar: GravatarService,
    ccxt: CcxtService,
    callback: Callback<Result<Profile, Error>>,
    profile: Option<Profile>,
    exchanges: Vec<String>,
    task: Option<FetchTask>,
}

pub enum Msg {
    Gravatar,
    GravatarReady(Result<Profile, Error>),
    Exchanges,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        Model {
            gravatar: GravatarService::new(),
            ccxt: CcxtService::new(),
            callback: link.send_back(Msg::GravatarReady),
            profile: None,
            exchanges: Vec::new(),
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Gravatar => {
                let task = self.gravatar.profile("205e460b479e2e5b48aec07710c08d50", self.callback.clone());
                self.task = Some(task);
            }
            Msg::GravatarReady(Ok(profile)) => {
                self.profile = Some(profile);
            }
            Msg::GravatarReady(Err(_)) => {
                // Can't load gravatar profile
            }
            Msg::Exchanges => {
                self.exchanges = self.ccxt.exchanges();
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
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
