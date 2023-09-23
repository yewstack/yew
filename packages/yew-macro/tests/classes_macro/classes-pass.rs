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

fn compile_pass() {
    // multiple literals
    ::yew::classes!("one", "two");
    // single literal
    ::yew::classes!("one");
    // empty
    ::yew::classes!();

    // multiple expressions
    ::yew::classes!(::std::vec!["one"], ::std::vec!["two"]);
    // single expression
    ::yew::classes!(::std::vec!["one", "two"]);

    // single array
    ::yew::classes!(["one", "two"]);
    // multiple arrays
    ::yew::classes!(["one"], ["two"]);

    // optional classes
    ::yew::classes!(
        ::std::option::Option::Some("one"),
        ::std::option::Option::None::<&'static ::std::primitive::str>,
    );

    // mixed types
    {
        use ::std::borrow::ToOwned;
        ::yew::classes!("one".to_owned(), "two", ::std::vec!["three"], ["four", "five"]);
    }
}

fn main() {}
