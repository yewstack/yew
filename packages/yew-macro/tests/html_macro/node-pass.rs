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

fn main() {
    (::yew::html! { "" }).unwrap();
    (::yew::html! { 'a' }).unwrap();
    (::yew::html! { "hello" }).unwrap();
    (::yew::html! { "42" }).unwrap();
    (::yew::html! { "1.234" }).unwrap();
    (::yew::html! { "true" }).unwrap();

    (::yew::html! { <span>{ "" }</span> }).unwrap();
    (::yew::html! { <span>{ 'a' }</span> }).unwrap();
    (::yew::html! { <span>{ "hello" }</span> }).unwrap();
    (::yew::html! { <span>{ "42" }</span> }).unwrap();
    (::yew::html! { <span>{ "1.234" }</span> }).unwrap();
    (::yew::html! { <span>{ "true" }</span> }).unwrap();

    (::yew::html! { ::std::format!("Hello") }).unwrap();
    (::yew::html! { ::std::string::ToString::to_string("Hello") }).unwrap();

    let msg = "Hello";
    (::yew::html! { msg }).unwrap();
}
