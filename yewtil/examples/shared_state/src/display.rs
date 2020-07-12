use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yewtil::state::{GlobalProps, SharedHandle, SharedStateComponent};
use yewtil::NeqAssign;

use crate::app::AppState;

pub struct Model {
    props: GlobalProps<AppState>,
}

impl Component for Model {
    type Message = ();
    type Properties = GlobalProps<AppState>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let name = &self.props.handle.state().user.name;
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
