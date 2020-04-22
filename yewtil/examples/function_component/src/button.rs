use yew::{html, Callback, MouseEvent, Html};
use yewtil::function_component;

#[function_component(Button)]
pub fn button(
    callback: &Callback<MouseEvent>,
    #[prop_or_default]
    text: String,
    #[prop_or_default]
    _num: usize,
) -> Html {
    html! {
        <button onclick=callback>{ text }</button>
    }
}
