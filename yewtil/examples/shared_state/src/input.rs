use yew::{html, Component, ComponentLink, Html, InputData, Properties, ShouldRender};
use yewtil::state::{Shared, SharedState, SharedStateComponent};
use yewtil::NeqAssign;

use crate::app::AppState;

#[derive(Clone, Properties, PartialEq)]
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
    SetName(String),
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
            Msg::SetName(name) => {
                self.props.state.reduce(|state| state.user.name = name);
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let input_value = &self.props.state.get().user.name;
        html! {
            <>
                <input
                    type="text"
                    value=input_value
                    placeholder="Enter your name"
                    // Using internal callback
                    oninput = self.link.callback(|data: InputData| Msg::SetName(data.value))
                    />
                <input
                    type="button"
                    value="Clear"
                    // Using provided callback
                    onclick = self.props.state.reduce_callback(|_, state|  state.user.name.clear())
                    />
            </>
        }
    }
}

pub type Input = SharedStateComponent<AppState, Model>;
