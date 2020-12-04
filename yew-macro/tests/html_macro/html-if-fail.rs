use yew::prelude::*;

fn compile_fail() {
    html! { if {} };
    html! { if 42 {} };
    html! { if true {} else };
    html! { if true {} else if {} };
    html! { if true {} else if true {} else };
    html! { if true {} else if true {} else };
}

fn main() {}
