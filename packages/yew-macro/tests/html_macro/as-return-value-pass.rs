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

pub struct MyComponent;
impl ::yew::Component for MyComponent {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &::yew::Context<Self>) -> Self {
        ::std::unimplemented!()
    }
    fn view(&self, _ctx: &::yew::Context<Self>) -> ::yew::Html {
        ::std::unimplemented!()
    }
}

// can test "unused braces" warning inside the macro
// https://github.com/yewstack/yew/issues/2157
fn make_my_component()-> ::yew::virtual_dom::VChild<MyComponent>{
    ::yew::html_nested!{<MyComponent/>}
}

// can test "unused braces" warning inside the macro
// https://github.com/yewstack/yew/issues/2157
fn make_my_component_html()-> ::yew::Html{
    ::yew::html!{<MyComponent/>}
}

fn main(){}