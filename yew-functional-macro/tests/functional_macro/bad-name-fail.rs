use yew::prelude::*;
use yew_functional::functional_component;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

#[functional_component(let)]
fn comp(props: &Props) -> Html {
    html! {
        <p>
            { props.a }
        </p>
    }
}

#[functional_component(x, y, z)]
fn comp_2(props: &Props) -> Html {
    html! {
        <p>
            { props.a }
        </p>
    }
}

#[functional_component(124)]
fn comp_3(props: &Props) -> Html {
    html! {
        <p>
            { props.a }
        </p>
    }
}

#[functional_component(component)]
fn component(props: &Props) -> Html {
    html! {
        <p>
            { props.a }
        </p>
    }
}

fn main() {}
