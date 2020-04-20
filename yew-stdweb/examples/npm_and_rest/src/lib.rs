#![recursion_limit = "128"]

#[macro_use]
extern crate stdweb;

// Own services implementation
pub mod ccxt;
pub mod gravatar;

use anyhow::Error;
use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};

use ccxt::CcxtService;
use gravatar::{GravatarService, Profile};

pub struct Model {
    link: ComponentLink<Self>,
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

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link: link.clone(),
            gravatar: GravatarService::new(),
            ccxt: CcxtService::new(),
            callback: link.callback(Msg::GravatarReady),
            profile: None,
            exchanges: Vec::new(),
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Gravatar => {
                let task = self
                    .gravatar
                    .profile("205e460b479e2e5b48aec07710c08d50", self.callback.clone());
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

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let view_exchange = |exchange| {
            html! {
                <li>{ exchange }</li>
            }
        };
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Exchanges)>{ "Get Exchanges" }</button>
                <button onclick=self.link.callback(|_| Msg::Gravatar)>{ "Get Gravatar" }</button>
                <ul>
                    { for self.exchanges.iter().map(view_exchange) }
                </ul>
            </div>
        }
    }
}
