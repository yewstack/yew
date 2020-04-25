use yew::{
    events::InputData, html, Callback, Component, ComponentLink, Html, Properties, ShouldRender,
};
use yewtil::ptr::Lrc;
use yewtil::NeqAssign;

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub text: Lrc<String>,
    pub callback: Callback<()>,
}

pub struct Child {
    props: Props,
    on_input: Callback<InputData>,
}

pub enum Msg {
    UpdateText(InputData),
}

impl Component for Child {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Child {
            props,
            on_input: link.callback(Msg::UpdateText),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateText(input) => {
                // Only update the Lrc if the new value is different.
                self.props.text.neq_set(input.value);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <>
                <input
                    type = "text"
                    value = self.props.text.as_ref()
                    oninput = &self.on_input
                />
                <button onclick=self.props.callback.reform(|_| ()) >
                    {"Update parent"}
                </button>
            </>
        }
    }
}
