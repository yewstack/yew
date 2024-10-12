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

pub struct Generic<T> {
    marker: ::std::marker::PhantomData<T>,
}

impl<T> ::yew::Component for Generic<T>
where
    T: 'static,
{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &::yew::Context<Self>) -> Self {
        ::std::unimplemented!()
    }
    fn view(&self, _ctx: &::yew::Context<Self>) -> ::yew::Html {
        ::std::unimplemented!()
    }
}

pub struct Generic2<T1, T2> {
    marker: ::std::marker::PhantomData<(T1, T2)>,
}

impl<T1, T2> ::yew::Component for Generic2<T1, T2>
where
    T1: 'static,
    T2: 'static,
{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &::yew::Context<Self>) -> Self {
        ::std::unimplemented!()
    }
    fn view(&self, _ctx: &::yew::Context<Self>) -> ::yew::Html {
        ::std::unimplemented!()
    }
}

fn compile_pass() {
    _ = ::yew::html! { <Generic<::std::string::String> /> };
    _ = ::yew::html! { <Generic<(u8, bool)> /> };
    _ = ::yew::html! { <Generic<(u8, bool)> ></Generic<(u8, bool)>> };
    _ = ::yew::html! { <Generic<::std::string::String> ></Generic<::std::string::String>> };

    _ = ::yew::html! { <Generic<::std::vec::Vec<::std::string::String>> /> };
    _ = ::yew::html! { <Generic<::std::vec::Vec<::std::string::String>>></ Generic<::std::vec::Vec<::std::string::String>>> };

    _ = ::yew::html! { <Generic<::std::primitive::usize> /> };
    _ = ::yew::html! { <Generic<::std::primitive::usize>></Generic<::std::primitive::usize>> };
    _ = ::yew::html! { <Generic<::std::string::String, > /> };
    _ = ::yew::html! { <Generic<::std::string::String, >></Generic<::std::string::String,>> };

    _ = ::yew::html! { <Generic2<::std::string::String, ::std::string::String> /> };
    _ = ::yew::html! { <Generic2<::std::string::String, ::std::string::String>></Generic2<::std::string::String, ::std::string::String>> };

    _ = ::yew::html! { <Generic2<::std::string::String, ::std::string::String, > /> };
    _ = ::yew::html! { <Generic2<::std::string::String, ::std::string::String, >></Generic2<::std::string::String, ::std::string::String, >> };
}

fn main() {}
