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
    (::yew::html! { <>{ "Hi" }</> }).unwrap();
    (::yew::html! { <>{ ::std::format!("Hello") }</> }).unwrap();
    (::yew::html! { <>{ ::std::string::ToString::to_string("Hello") }</> }).unwrap();

    let msg = "Hello";
    (::yew::html! { <div>{ msg }</div> }).unwrap();

    let subview = ::yew::html! { "subview!" };
    (::yew::html! { <div>{ subview }</div> }).unwrap();

    let subview = || ::yew::html! { "subview!" };
    (::yew::html! { <div>{ subview() }</div> }).unwrap();

    (::yew::html! {
        <ul>
            { for ::std::iter::Iterator::map(0..3, |num| { ::yew::html! { <span>{ num }</span> }}) }
        </ul>
    })
    .unwrap();

    let item = |num| ::yew::html! { <li>{ ::std::format!("item {}!", num) }</li> };
    (::yew::html! {
        <ul>
            { for ::std::iter::Iterator::map(0..3, item) }
        </ul>
    })
    .unwrap();
}
