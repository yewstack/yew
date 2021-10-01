use web_sys::HtmlInputElement;
use yew::events::KeyboardEvent;
use yew::{function_component, html, Callback, Properties, TargetCast};

#[derive(PartialEq, Properties, Clone)]
pub struct InputProps {
    pub add: Callback<String>,
}

#[function_component(Input)]
pub fn entry_edit(props: &InputProps) -> Html {
    let onkeypress = {
        let add = props.add.clone();

        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();

                input.set_value("");
                add.emit(value);
            }
        }
    };

    html! {
        <input
            class="new-todo"
            placeholder="What needs to be done?"
            {onkeypress}
        />
    }
}
