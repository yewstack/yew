#![no_implicit_prelude]

#[derive(::yew::prelude::Properties, ::std::prelude::rust_2021::PartialEq,)]
struct Props {
    a: usize,
}

#[::yew::prelude::function_component]
fn Comp(props: &Props) -> ::yew::prelude::Html {
    ::yew::prelude::html! {
        <p>
            { props.a }
        </p>
    }
}

fn main() {
    let _ = ::yew::prelude::html! {
        <Comp a={0} />
    };
}
