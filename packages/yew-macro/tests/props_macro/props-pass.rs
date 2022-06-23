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
    #[prop_or_default]
    b: ::std::primitive::usize,
}

fn pass_simple_props() {
    ::yew::props!(Props { a: 5 });
    let (a, b) = (3, 5);
    ::yew::props!(Props { a, b });
}

#[derive(::yew::Properties, ::std::cmp::PartialEq)]
pub struct RawIdentProps {
    r#true: ::std::primitive::usize,
    #[prop_or_default]
    r#pointless_raw_name: ::std::primitive::usize,
}

fn pass_raw_idents() {
    ::yew::props!(RawIdentProps { r#true: 5 });
    let (r#true, r#pointless_raw_name) = (3, 5);
    ::yew::props!(RawIdentProps { r#true, r#pointless_raw_name });
}

#[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
struct BuildProp {
    build: ::std::primitive::usize,
}

fn pass_build_prop() {
    ::yew::props!(BuildProp { build: 5 });
}

#[derive(::yew::Properties, ::std::cmp::PartialEq)]
struct GenericProps<T: ::std::cmp::PartialEq> {
    item: T,
}

fn pass_generic_props<T: ::std::cmp::PartialEq>(the_item: T) {
    ::yew::props!(GenericProps<T> { item: the_item });
}

fn main() {}
