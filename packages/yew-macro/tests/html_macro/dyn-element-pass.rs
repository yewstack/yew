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
    let dyn_tag = || ::std::string::ToString::to_string("test");
    let mut next_extra_tag = {
        let mut it = ::std::iter::IntoIterator::into_iter(::std::vec!["a", "b"]);
        move || ::std::option::Option::unwrap(::std::iter::Iterator::next(&mut it))
    };

    ::yew::html! {
        <@{ dyn_tag() }>
            <@{ next_extra_tag() } class="extra-a"/>
            <@{ next_extra_tag() } class="extra-b"/>
        </@>
    };

    ::yew::html! {
        <@{
            let tag = dyn_tag();
            if tag == "test" {
                "div"
            } else {
                "a"
            }
        }/>
    };
}
