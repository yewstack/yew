use yew::prelude::*;
use yew_functional::functional_component;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

#[functional_component(Comp1)]
fn comp_1(props: &Props) {
    let _x = html! {
        <p>
            { props.a }
        </p>
    };
}

#[functional_component(Comp)]
fn comp(props: &Props) -> u32 {
    let _x = html! {
        <p>
            { props.a }
        </p>
    };
    1
}

fn main() {}
