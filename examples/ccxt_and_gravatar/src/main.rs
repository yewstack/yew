mod ccxt;
mod gravatar;

use anyhow::Error;
use gravatar::Profile;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

pub enum Msg {
    Gravatar,
    GravatarReady(Result<Profile, Error>),
    Exchanges,
}

pub struct Model {
    link: ComponentLink<Self>,
    profile: Option<Profile>,
    exchanges: Vec<String>,
    task: Option<FetchTask>,
}
impl Model {
    fn view_profile(&self) -> Html {
        if let Some(profile) = &self.profile {
            html! {
                <>
                    <h2>{ "Gravatar" }</h2>
                    <pre>{ format!("{:#?}", profile) }</pre>
                </>
            }
        } else {
            html! {}
        }
    }

    fn view_exchanges(&self) -> Html {
        if self.exchanges.is_empty() {
            html! {}
        } else {
            html! {
                <>
                    <h2>{ "Exchanges" }</h2>
                    <ul>
                        { for self.exchanges.iter().map(|exchange| html! { <li>{ exchange }</li> }) }
                    </ul>
                </>
            }
        }
    }
}
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            profile: None,
            exchanges: Vec::new(),
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Gravatar => {
                let task = gravatar::fetch_profile(
                    "205e460b479e2e5b48aec07710c08d50",
                    self.link.callback(Msg::GravatarReady),
                );
                self.task = Some(task);
                true
            }
            Msg::GravatarReady(Ok(profile)) => {
                self.profile = Some(profile);
                true
            }
            Msg::GravatarReady(Err(_)) => {
                // Can't load gravatar profile
                false
            }
            Msg::Exchanges => {
                self.exchanges = ccxt::iter_exchanges().collect();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Exchanges)>{ "Get Exchanges" }</button>
                <button onclick=self.link.callback(|_| Msg::Gravatar)>{ "Get Gravatar" }</button>

                { self.view_profile() }
                { self.view_exchanges() }
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
