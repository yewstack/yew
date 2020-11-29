use yew::prelude::*;

fn compile_pass() {
    html! { if true {} };
    html! { if true { <div/> } };
    html! { if true { <><div/><div/></> } };
    html! { if true { { html!() } } };
    html! { if true { { { let _x = 42; html!() } } } };
    html! { if true {} else {} };
    html! { if false {} else if true {} else {} };
    html! { if let Some(text) = Some("text") { <span>{text}</span> } };
}

fn main() {}
