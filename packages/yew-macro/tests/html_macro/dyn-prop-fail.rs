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

#[derive(::yew::Properties, ::std::cmp::PartialEq)]
pub struct SimpleProps {
    pub test: ::std::string::String,
}

pub struct Simple;
impl ::yew::Component for Simple {
    type Message = ();
    type Properties = SimpleProps;

    fn create(_ctx: &::yew::Context<Self>) -> Self {
        ::std::unimplemented!()
    }

    fn view(&self, _ctx: &::yew::Context<Self>) -> ::yew::Html {
        ::std::unimplemented!()
    }
}

pub struct Fail;

fn main() {
    _ = ::yew::html! { <span { Fail }={ "" } /> };

    let dyn_prop = || Fail;
    _ = ::yew::html! { <span { dyn_prop() }={ "" } /> };
    
    _ = ::yew::html! { <Simple { "test" }={ "" } /> }
}
