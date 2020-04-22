use yew::{
    events::InputData, html, Callback, Component, ComponentLink, Html, Properties, ShouldRender,
};
use yewtil::ptr::Irc;
use yewtil::NeqAssign;

#[derive(PartialEq, Properties)]
pub struct Props {
    /// This value can't be altered.
    #[props(required)]
    pub text: Irc<String>,
    /// This heavily implies the only way to update the text field is to send a message back
    /// to the parent to have the parent component update it.
    #[props(required)]
    pub callback: Callback<String>,
}

pub struct Child {
    props: Props,
}

pub enum Msg {
    UpdateText(InputData),
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
                self.props.callback.emit(input.value);
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
                    value = &*self.props.text,
                    oninput = |x| Msg::UpdateText(x)
                />
            </>
        }
    }
}
