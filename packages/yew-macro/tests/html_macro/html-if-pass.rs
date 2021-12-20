use yew::prelude::*;

fn compile_pass_lit() {
    html! { if true {} };
    html! { if true { <div/> } };
    html! { if true { <div/><div/> } };
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

fn compile_pass_expr() {
    let condition = true;

    html! { if condition {} };
    html! { if condition { <div/> } };
    html! { if condition { <div/><div/> } };
    html! { if condition { <><div/><div/></> } };
    html! { if condition { { html! {} } } };
    html! { if condition { { { let _x = 42; html! {} } } } };
    html! { if condition {} else {} };
    html! { if condition {} else if condition {} };
    html! { if condition {} else if condition {} else {} };
    html! { if let Some(text) = Some("text") { <span>{ text }</span> } };
    html! { <><div/>if condition {}<div/></> };
    html! { <div>if condition {}</div> };
}

fn main() {}
