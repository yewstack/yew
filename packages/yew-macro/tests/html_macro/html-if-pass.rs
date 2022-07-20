#![no_implicit_prelude]

fn compile_pass_lit() {
    ::yew::html! { if true {} };
    ::yew::html! { if true { <div/> } };
    ::yew::html! { if true { <div/><div/> } };
    ::yew::html! { if true { <><div/><div/></> } };
    ::yew::html! { if true { { ::yew::html! {} } } };
    ::yew::html! { if true { { { let _x = 42; ::yew::html! {} } } } };
    ::yew::html! { if true {} else {} };
    ::yew::html! { if true {} else if true {} };
    ::yew::html! { if true {} else if true {} else {} };
    ::yew::html! { if let ::std::option::Option::Some(text) = ::std::option::Option::Some("text") { <span>{ text }</span> } };
    ::yew::html! { <><div/>if true {}<div/></> };
    ::yew::html! { <div>if true {}</div> };
}

fn compile_pass_expr() {
    let condition = true;

    ::yew::html! { if condition {} };
    ::yew::html! { if condition { <div/> } };
    ::yew::html! { if condition { <div/><div/> } };
    ::yew::html! { if condition { <><div/><div/></> } };
    ::yew::html! { if condition { { ::yew::html! {} } } };
    ::yew::html! { if condition { { { let _x = 42; ::yew::html! {} } } } };
    ::yew::html! { if condition {} else {} };
    ::yew::html! { if condition {} else if condition {} };
    ::yew::html! { if condition {} else if condition {} else {} };
    ::yew::html! { if let ::std::option::Option::Some(text) = ::std::option::Option::Some("text") { <span>{ text }</span> } };
    ::yew::html! { <><div/>if condition {}<div/></> };
    ::yew::html! { <div>if condition {}</div> };
}

fn main() {}
