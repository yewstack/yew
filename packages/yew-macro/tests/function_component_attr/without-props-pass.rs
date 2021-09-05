#![no_implicit_prelude]

#[::yew::function_component(Comp)]
fn comp() -> ::yew::Html {
    ::yew::html! {
        <p>
            { "Test" }
        </p>
    }
}

fn main() {}
