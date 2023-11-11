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

fn empty_vec() -> ::std::vec::Vec<::yew::Html> {
    ::std::vec::Vec::<::yew::Html>::new()
}

fn empty_iter() -> impl ::std::iter::Iterator<Item = ::yew::Html> {
    ::std::iter::empty::<::yew::Html>()
}

fn main() {
    _ = ::yew::html! { for empty_iter() };
    _ = ::yew::html! { for { empty_iter() } };

    let empty = empty_vec();
    _ = ::yew::html! { for empty };

    _ = ::yew::html! { for empty_vec() };
    _ = ::yew::html! { for ::std::iter::IntoIterator::into_iter(empty_vec()) };
    _ = ::yew::html! { for ::std::iter::Iterator::map(0..3, |num| { ::yew::html! { <span>{ num }</span> } }) };
}
