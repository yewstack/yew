use yew::prelude::*;

#[hook]
pub fn use_some_macro_inner(val: &str) -> String {
    use_state(|| val.to_owned()).to_string()
}

macro_rules! use_some_macro {
    () => {
        use_some_macro_inner("default str")
    };
    ($t: tt) => {
        use_some_macro_inner($t)
    };
}

#[function_component]
fn Comp() -> Html {
    let a = use_some_macro!();
    let b = use_some_macro!("b");

    html! {
        <div>{a}{b}</div>
    }
}

fn main() {}
