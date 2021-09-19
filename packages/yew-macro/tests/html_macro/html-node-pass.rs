#![no_implicit_prelude]

// Shadow primitives
#[allow(non_camel_case_types)]
pub struct bool;
#[allow(non_camel_case_types)]
pub struct char;
#[allow(non_camel_case_types)]
pub struct f32;
#[allow(non_camel_case_types)]
pub struct f64;
#[allow(non_camel_case_types)]
pub struct i128;
#[allow(non_camel_case_types)]
pub struct i16;
#[allow(non_camel_case_types)]
pub struct i32;
#[allow(non_camel_case_types)]
pub struct i64;
#[allow(non_camel_case_types)]
pub struct i8;
#[allow(non_camel_case_types)]
pub struct isize;
#[allow(non_camel_case_types)]
pub struct str;
#[allow(non_camel_case_types)]
pub struct u128;
#[allow(non_camel_case_types)]
pub struct u16;
#[allow(non_camel_case_types)]
pub struct u32;
#[allow(non_camel_case_types)]
pub struct u64;
#[allow(non_camel_case_types)]
pub struct u8;
#[allow(non_camel_case_types)]
pub struct usize;

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
    ::yew::html! { {<::std::string::String as ::std::convert::From<&::std::primitive::str>>::from("Hello") } };

    let msg = "Hello";
    ::yew::html! { msg };
}

fn main() {}
