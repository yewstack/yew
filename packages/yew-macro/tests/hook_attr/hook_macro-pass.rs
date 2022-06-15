#![no_implicit_prelude]

#[::yew::functional::hook]
pub fn use_some_macro_inner(val: &str) -> ::std::string::String {
    let state = ::yew::functional::use_state(|| ::std::borrow::ToOwned::to_owned(val));
    ::std::string::ToString::to_string(&*state)
}

macro_rules! use_some_macro {
    () => {
        use_some_macro_inner("default str")
    };
    ($t: tt) => {
        use_some_macro_inner($t)
    };
}

#[::yew::functional::function_component]
fn Comp() -> ::yew::Html {
    let a = use_some_macro!();
    let b = use_some_macro!("b");

    ::yew::html! {
        <div>{a}{b}</div>
    }
}

fn main() {}
