use yew::prelude::*;

fn compile_fail() {
    html! { if {} };
    html! { if 42 {} };
    html! { if true {} else };
    html! { if true {} else if {} };
    html! { if true {} else if true {} else };
    html! { if true {} else if true {} else };
}

fn unnecessary_fragment() {
    // Fragment in if body
    html! { if true { <><div/><div/></> } };

    // Fragment in else body
    html! { if true { <div/> } else { <><span/><span/></> } };
}

fn main() {}
