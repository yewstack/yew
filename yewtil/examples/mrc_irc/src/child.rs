use yew::{
    events::InputData, html, Callback, Component, ComponentLink, Html, Properties, ShouldRender,
};
use yewtil::ptr::Irc;
use yewtil::NeqAssign;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// This value can't be altered.
    pub text: Irc<String>,
    /// This heavily implies the only way to update the text field is to send a message back
    /// to the parent to have the parent component update it.
    pub callback: Callback<String>,
}

pub struct Child {
    props: Props,
}

impl Component for Child {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Child { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <>
                <input
                    type = "text"
                    value = &*self.props.text
                    oninput = self.props.callback.reform(|i: InputData| i.value)
                />
            </>
        }
    }
}
