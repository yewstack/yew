use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};
use yewtil::state::{Shared, SharedState};

use crate::app::{AppState, User};

pub struct Model {
    state: Shared<AppState>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Login(String),
    Logout,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Shared<AppState>;

    fn create(state: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { state, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Login(name) => {
                self.state
                    .set_with(move |state| state.user = Some(User { name }));
                false
            }
            Msg::Logout => {
                self.state.set_with(move |state| state.user = None);
                false
            }
        }
    }

    fn change(&mut self, state: Self::Properties) -> ShouldRender {
        if self.state.get().user != state.get().user {
            self.state = state;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        if self.state.get().user.is_none() {
            html! {
                <input type="text" placeholder="Enter your name" oninput = self.link.callback(|data: InputData| Msg::Login(data.value)) />
            }
        } else {
            html! {
                <input type="button" value="Logout" onclick = self.link.callback(|_| Msg::Logout) />
            }
        }
    }
}

pub type Input = SharedState<AppState, Model>;
