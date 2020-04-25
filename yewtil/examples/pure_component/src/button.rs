use yew::virtual_dom::VNode;
use yew::{html, Callback, MouseEvent, Properties};
use yewtil::{Pure, PureComponent};

/// Alias to make usability better.
pub type Button = Pure<PureButton>;

#[derive(Clone, PartialEq, Properties)]
pub struct PureButton {
    pub callback: Callback<MouseEvent>,
    #[prop_or_default]
    pub text: String,
}

impl PureComponent for PureButton {
    fn render(&self) -> VNode {
        html! {
            <button onclick=&self.callback>{ &self.text }</button>
        }
    }
}
