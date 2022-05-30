use implicit_clone::unsync::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct FolksViewProps {
    folks: IArray<IString>,
}

#[function_component(FolksView)]
fn folks_view(props: &FolksViewProps) -> Html {
    html! {
        <>
        <p>{"Hello to:"}</p>
        <ul>
        { for props.folks.iter().map(|s| html!(<li>{s}</li>)) }
        </ul>
        </>
    }
}

#[function_component(ArrayExample)]
pub fn array_example() -> Html {
    let folks = use_state(IArray::<IString>::default);
    let onkeyup = {
        let folks = folks.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let event: Event = e.dyn_into().unwrap_throw();
                let event_target = event.target().unwrap_throw();
                let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
                let name = target.value();
                target.set_value("");

                folks.set(
                    folks
                        .iter()
                        .chain(std::iter::once(IString::from(name)))
                        .collect(),
                );
            }
        })
    };

    html! {
        <>
        <h2>{"Input"}</h2>
        <input {onkeyup} />
        <h2>{"Output"}</h2>
        <FolksView folks={&*folks} />
        </>
    }
}
