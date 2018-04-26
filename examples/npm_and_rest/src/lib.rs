extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

// Own services implementation
pub mod gravatar;
pub mod ccxt;

use failure::Error;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

use gravatar::{GravatarService, Profile};
use ccxt::CcxtService;

pub struct Model {
    profile: Option<Profile>,
    exchanges: Vec<String>,
    task: Option<FetchTask>,
}

pub enum Msg {
    Gravatar,
    GravatarReady(Result<Profile, Error>),
    Exchanges,
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<GravatarService> + AsMut<CcxtService> + 'static,
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {
            profile: None,
            exchanges: Vec::new(),
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::Gravatar => {
                let callback = env.send_back(Msg::GravatarReady);
                let gravatar: &mut GravatarService = env.as_mut();
                let task = gravatar.profile("205e460b479e2e5b48aec07710c08d50", callback);
                self.task = Some(task);
            }
            Msg::GravatarReady(Ok(profile)) => {
                self.profile = Some(profile);
            }
            Msg::GravatarReady(Err(_)) => {
                // Can't load gravatar profile
            }
            Msg::Exchanges => {
                let ccxt: &mut CcxtService = env.as_mut();
                self.exchanges = ccxt.exchanges();
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<GravatarService> + AsMut<CcxtService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
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
