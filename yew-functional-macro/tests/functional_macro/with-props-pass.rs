#![no_implicit_prelude]

#[derive(Clone, ::yew::Properties, PartialEq)]
struct Props {
    a: usize,
}

#[::yew_functional::functional_component(Comp)]
fn comp(props: &Props) -> ::yew::Html {
    ::yew::html! {
        <p>
            { props.a }
        </p>
    }
}

fn main() {}
