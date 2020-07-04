use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yewtil::state::{Shared, SharedState};

use crate::app::AppState;

pub struct Model {
    state: Shared<AppState>,
}

impl Component for Model {
    type Message = ();
    type Properties = Shared<AppState>;

    fn create(state: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model { state }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
        self.state
            .get()
            .user
            .as_ref()
            .map(|user| {
                html! {
                    <p>{ format!("Hi {}", user.name) }</p>
                }
            })
            .unwrap_or_else(|| {
                html! {
                    <p>{"Please enter your name"}</p>
                }
            })
    }
}

pub type Display = SharedState<AppState, Model>;
