use yew::prelude::*;

fn compile_pass_lit() {
    (html! { if true {} }).unwrap();
    (html! { if true { <div/> } }).unwrap();
    (html! { if true { <div/><div/> } }).unwrap();
    (html! { if true { <><div/><div/></> } }).unwrap();
    (html! { if true { { html! {} } } }).unwrap();
    (html! { if true { { { let _x = 42; html! {} } } } }).unwrap();
    (html! { if true {} else {} }).unwrap();
    (html! { if true {} else if true {} }).unwrap();
    (html! { if true {} else if true {} else {} }).unwrap();
    (html! { if let Some(text) = Some("text") { <span>{ text }</span> } }).unwrap();
    (html! { <><div/>if true {}<div/></> }).unwrap();
    (html! { <div>if true {}</div> }).unwrap();
}

fn compile_pass_expr() {
    let condition = true;

    (html! { if condition {} }).unwrap();
    (html! { if condition { <div/> } }).unwrap();
    (html! { if condition { <div/><div/> } }).unwrap();
    (html! { if condition { <><div/><div/></> } }).unwrap();
    (html! { if condition { { html! {} } } }).unwrap();
    (html! { if condition { { { let _x = 42; html! {} } } } }).unwrap();
    (html! { if condition {} else {} }).unwrap();
    (html! { if condition {} else if condition {} }).unwrap();
    (html! { if condition {} else if condition {} else {} }).unwrap();
    (html! { if let Some(text) = Some("text") { <span>{ text }</span> } }).unwrap();
    (html! { <><div/>if condition {}<div/></> }).unwrap();
    (html! { <div>if condition {}</div> }).unwrap();
}

fn main() {}
