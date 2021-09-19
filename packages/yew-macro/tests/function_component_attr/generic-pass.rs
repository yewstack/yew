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
    a: ::std::primitive::usize,
}

#[::yew::function_component(Comp)]
fn comp<P>(_props: &P) -> ::yew::Html
where
    P: ::yew::Properties + ::std::cmp::PartialEq,
{
    ::yew::html! {
        <p></p>
    }
}

#[::yew::function_component(Comp1)]
fn comp1<T1, T2>(_props: &()) -> ::yew::Html {
    ::yew::html! {
        <p></p>
    }
}

#[::yew::function_component(ConstGenerics)]
fn const_generics<const N: ::std::primitive::i32>() -> ::yew::Html {
    ::yew::html! {
        <div>
            { N }
        </div>
    }
}

fn compile_pass() {
    ::yew::html! { <Comp<Props> a=10 /> };
    ::yew::html! { <Comp1<::std::primitive::usize, ::std::primitive::usize> /> };

    ::yew::html! { <ConstGenerics<10> /> };
}

fn main() {}
