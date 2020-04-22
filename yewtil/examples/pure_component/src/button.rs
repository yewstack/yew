use yew::virtual_dom::VNode;
use yew::{html, Callback, ClickEvent, Properties};
use yewtil::{Pure, PureComponent};

/// Alias to make usability better.
pub type Button = Pure<PureButton>;

#[derive(PartialEq, Properties)]
pub struct PureButton {
    #[props(required)]
    pub callback: Callback<ClickEvent>,
    pub text: String,
}

impl PureComponent for PureButton {
    fn render(&self) -> VNode {
        html! {
            <button onclick=&self.callback>{ &self.text }</button>
        }
    }
}
