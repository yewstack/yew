use yew::{html, Component, ComponentLink, Html, InputData, Properties, ShouldRender};
use yewtil::state::{Shared, SharedState, SharedStateComponent};

use crate::app::AppState;

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub state: Shared<AppState>,
    pub max_len: usize,
}

impl SharedState for Props {
    type State = AppState;

    fn shared_state(&mut self) -> &mut Shared<Self::State> {
        &mut self.state
    }
}

pub enum Msg {
    SetUser(String),
    Clear,
}

pub struct Model {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetUser(name) => {
                let name = if name.len() <= self.props.max_len {
                    name
                } else {
                    "your name is too long".to_string()
                };
                self.props
                    .state
                    .reduce(move |state| state.user.name = name);
                false
            }
            Msg::Clear => {
                self.props
                    .state
                    .reduce(move |state| state.user.name.clear());
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.state.get().user != props.state.get().user {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let input_value = &self.props.state.get().user.name;
        html! {
            <>
                <input type="text" value = input_value placeholder="Enter your name" oninput = self.link.callback(|data: InputData| Msg::SetUser(data.value)) />
                <input type="button" value="Clear" onclick = self.link.callback(|_| Msg::Clear) />
            </>
        }
    }
}

pub type Input = SharedStateComponent<AppState, Model, Props>;
