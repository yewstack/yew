use yew::prelude::*;
use yew_functional::function_component;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

#[function_component(let)]
fn comp(props: &Props) -> Html {
    html! {
        <p>
            { props.a }
        </p>
    }
}

#[function_component(x, y, z)]
fn comp_2(props: &Props) -> Html {
    html! {
        <p>
            { props.a }
        </p>
    }
}

#[function_component(124)]
fn comp_3(props: &Props) -> Html {
    html! {
        <p>
            { props.a }
        </p>
    }
}

#[function_component(component)]
fn component(props: &Props) -> Html {
    html! {
        <p>
            { props.a }
        </p>
    }
}

fn main() {}
