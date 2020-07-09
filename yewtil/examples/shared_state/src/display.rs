use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yewtil::state::{Shared, SharedStateComponent};
use yewtil::NeqAssign;

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
        self.state.neq_assign(state)
    }

    fn view(&self) -> Html {
        let name = &self.state.get().user.name;
        let name = if name.is_empty() {
            "Stranger".to_string()
        } else {
            name.clone()
        };

        html! {
            <p>{ format!("Hello, {}!", name) }</p>
        }
    }
}

pub type Display = SharedStateComponent<Model>;
