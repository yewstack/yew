#![no_implicit_prelude]

fn compile_pass_lit() {
    _ = ::yew::html! { if true {} };
    _ = ::yew::html! { if true { <div/> } };
    _ = ::yew::html! { if true { <div/><div/> } };
    _ = ::yew::html! { if true { <><div/><div/></> } };
    _ = ::yew::html! { if true { { ::yew::html! {} } } };
    _ = ::yew::html! { if true { { { let _x = 42; ::yew::html! {} } } } };
    _ = ::yew::html! { if true {} else {} };
    _ = ::yew::html! { if true {} else if true {} };
    _ = ::yew::html! { if true {} else if true {} else {} };
    _ = ::yew::html! { if let ::std::option::Option::Some(text) = ::std::option::Option::Some("text") { <span>{ text }</span> } };
    _ = ::yew::html! { <><div/>if true {}<div/></> };
    _ = ::yew::html! { <div>if true {}</div> };
}

fn compile_pass_expr() {
    let condition = true;

    _ = ::yew::html! { if condition {} };
    _ = ::yew::html! { if condition { <div/> } };
    _ = ::yew::html! { if condition { <div/><div/> } };
    _ = ::yew::html! { if condition { <><div/><div/></> } };
    _ = ::yew::html! { if condition { { ::yew::html! {} } } };
    _ = ::yew::html! { if condition { { { let _x = 42; ::yew::html! {} } } } };
    _ = ::yew::html! { if condition {} else {} };
    _ = ::yew::html! { if condition {} else if condition {} };
    _ = ::yew::html! { if condition {} else if condition {} else {} };
    _ = ::yew::html! { if let ::std::option::Option::Some(text) = ::std::option::Option::Some("text") { <span>{ text }</span> } };
    _ = ::yew::html! { <><div/>if condition {}<div/></> };
    _ = ::yew::html! { <div>if condition {}</div> };
}

fn main() {}
