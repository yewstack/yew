use yew::prelude::*;

fn compile_pass() {
    html! { if true {} };
    html! { if true { <div/> } };
    html! { if true { <><div/><div/></> } };
    html! { if true { { html! {} } } };
    html! { if true { { { let _x = 42; html! {} } } } };
    html! { if true {} else {} };
    html! { if true {} else if true {} };
    html! { if true {} else if true {} else {} };
    html! { if let Some(text) = Some("text") { <span>{ text }</span> } };
    html! { <><div/>if true {}<div/></> };
    html! { <div>if true {}</div> };
}

fn main() {}
