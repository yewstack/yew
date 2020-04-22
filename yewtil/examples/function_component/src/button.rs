use yew::{html, Callback, ClickEvent, Html};
use yewtil::function_component;

#[function_component(Button)]
pub fn button(
    #[props(required)] callback: &Callback<ClickEvent>,
    text: &String,
    _num: usize,
) -> Html {
    html! {
        <button onclick=callback>{ text }</button>
    }
}
