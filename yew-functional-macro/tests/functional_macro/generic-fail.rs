use yew::prelude::*;
use yew_functional::functional_component;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

#[functional_component(Comp)]
const fn comp<P: Properties>(props: &P) -> Html {
    html! {
        <p>
            { props.a }
        </p>
    }
}

fn main() {}
