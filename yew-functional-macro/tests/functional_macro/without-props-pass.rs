#![no_implicit_prelude]

#[::yew_functional::functional_component(Comp)]
fn comp() -> ::yew::Html {
    ::yew::html! {
        <p>
            { "Test" }
        </p>
    }
}

fn main() {}
