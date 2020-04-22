use yew::{
    events::InputData, html, Callback, Component, ComponentLink, Html, Properties, ShouldRender,
};
use yewtil::ptr::Lrc;
use yewtil::NeqAssign;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub text: Lrc<String>,
    #[props(required)]
    pub callback: Callback<()>,
}

pub struct Child {
    props: Props,
}

pub enum Msg {
    UpdateText(InputData),
    SendCallback,
}

impl Component for Child {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Child { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateText(input) => {
                // Only update the Lrc if the new value is different.
                self.props.text.neq_set(input.value);
                true
            }
            Msg::SendCallback => {
                self.props.callback.emit(());
                false
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
                    value = self.props.text.as_ref(),
                    oninput = |x| Msg::UpdateText(x)
                />
                <button onclick=|_| Msg::SendCallback >{"Update parent"} </button>
            </>
        }
    }
}
