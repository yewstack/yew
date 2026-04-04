use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

#[component(let)]
fn comp(props: &Props) -> Html {
    html! {
        <p>
            { props.a }
        </p>
    }
}

#[component(x, y, z)]
fn comp_2(props: &Props) -> Html {
    html! {
        <p>
            { props.a }
        </p>
    }
}

#[component(124)]
fn comp_3(props: &Props) -> Html {
    html! {
        <p>
            { props.a }
        </p>
    }
}

#[component(component)]
fn component(props: &Props) -> Html {
    html! {
        <p>
            { props.a }
        </p>
    }
}

fn main() {}
