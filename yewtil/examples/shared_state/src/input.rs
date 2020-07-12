use yew::{html, Component, ComponentLink, Html, InputData, Properties, ShouldRender};
use yewtil::state::{GlobalHandle, SharedHandle, SharedState, SharedStateComponent};
use yewtil::NeqAssign;

use crate::app::AppState;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub handle: GlobalHandle<AppState>,
}

impl SharedState for Props {
    type Handle = GlobalHandle<AppState>;

    fn handle(&mut self) -> &mut Self::Handle {
        &mut self.handle
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
                self.props.handle.reduce(|state| state.user.name = name);
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let input_value = &self.props.handle.state().user.name;
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
                    onclick = self.props.handle.reduce_callback(|state|  state.user.name.clear())
                    />
            </>
        }
    }
}

pub type Input = SharedStateComponent<Model>;
