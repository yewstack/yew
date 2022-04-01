use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_hover: Callback<()>,
    #[prop_or_default]
    pub ref_: HtmlRef<HtmlInputElement>,
}

#[function_component]
pub fn InputComponent(props: &Props) -> Html {
    let on_mouse_over = {
        let on_hover = props.on_hover.clone();
        Callback::from(move |_| on_hover.emit(()))
    };

    html! {
        <input
            type="text"
            class="input-component"
            onmouseover={on_mouse_over}
            ref={props.ref_.clone()}
        />
    }
}
