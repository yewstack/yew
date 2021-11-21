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

#[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
struct Props {
    n: ::std::primitive::i32,
}

struct MyComp;
impl ::yew::Component for MyComp {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &::yew::Context<Self>) -> Self {
        ::std::unimplemented!()
    }
    fn view(&self, _ctx: &::yew::Context<Self>) -> ::yew::Html {
        ::std::unimplemented!()
    }
}

fn compile_pass() {
    ::yew::props!(Props { n: 1 });
    ::yew::props!(self::Props { n: 1 });
    ::yew::props!(MyComp::Properties { n: 2 });
    ::yew::props!(self::MyComp::Properties { n: 3 });
    ::yew::props!(<MyComp as ::yew::Component>::Properties { n: 5});
}

fn main() {}
