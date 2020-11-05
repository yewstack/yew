use yew::prelude::*;
use yew_functional::{functional_component};

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

#[functional_component(Comp)]
fn comp(props: &Props, invalid: String) -> Html {
    html! {
        <p>
            { props.a }
            { invalid }
        </p>
    }
}


fn main() {}
