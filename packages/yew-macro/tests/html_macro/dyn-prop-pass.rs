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
    // basic example from https://htmx.org/attributes/hx-on/

    // repeating attrs is not valid HTMX nor HTML,
    // but valid html! (any checks can happen only during runtime)

    // literal
    _ = ::yew::html! { <span { "hx-on:click" }={ "alert('Clicked!')" } /> };
    _ = ::yew::html! { <span { "hx-on:click" }={ "alert('Clicked!')" } { "hx-on:click" }={ "alert('Clicked!')" } /> };

    // expr
    let dyn_prop = || ::std::string::ToString::to_string("hx-on:click");
    _ = ::yew::html! { <span { dyn_prop() }={ "alert('Clicked!')" } /> };
    _ = ::yew::html! { <span { dyn_prop() }={ "alert('Clicked!')" } { dyn_prop() }={ "alert('Clicked!')" } /> };
}
