#![no_implicit_prelude]

fn compile_pass() {
    ::yew::html! { "" };
    ::yew::html! { 'a' };
    ::yew::html! { "hello" };
    ::yew::html! { 42 };
    ::yew::html! { 1.234 };

    ::yew::html! { <span>{ "" }</span> };
    ::yew::html! { <span>{ 'a' }</span> };
    ::yew::html! { <span>{ "hello" }</span> };
    ::yew::html! { <span>{ 42 }</span> };
    ::yew::html! { <span>{ 1.234 }</span> };

    ::yew::html! { ::std::format!("Hello") };
    ::yew::html! { {<::std::string::String as ::std::convert::From<&str>>::from("Hello") } };

    let msg = "Hello";
    ::yew::html! { msg };
}

fn main() {}
