use yew::prelude::*;
use yew_function_components::function_component;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

#[function_component()]
fn comp(props: &Props) -> Html {
    html! {
        <p>
            { props.a }
        </p>
    }
}

fn main() {}
