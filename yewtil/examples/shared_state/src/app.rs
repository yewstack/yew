use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::{display, input};

#[derive(Clone, PartialEq, Default)]
pub struct User {
    pub name: String,
}

#[derive(Clone, PartialEq, Default)]
pub struct AppState {
    pub user: User,
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <display::Display />
                // Components can share state from anywhere!
                <div>
                    <input::Input />
                </div>
            </>
        }
    }
}
