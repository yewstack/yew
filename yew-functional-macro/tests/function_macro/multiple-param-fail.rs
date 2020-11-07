use yew::prelude::*;
use yew_functional::function_component;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

#[function_component(Comp)]
fn comp(props: &Props, invalid: String) -> Html {
    html! {
        <p>
            { props.a }
            { invalid }
        </p>
    }
}

#[function_component(Comp)]
fn comp3(props: &Props, invalid: String, another_invalid: u32) -> Html {
    html! {
        <p>
            { props.a }
            { invalid }
            { another_invalid }
        </p>
    }
}

fn main() {}
