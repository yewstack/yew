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
    _ = ::yew::html!{
        for i in 0 .. 10 {
            <span>{i}</span>
        }
    };

    struct Pair {
        value1: &'static ::std::primitive::str,
        value2: ::std::primitive::i32
    }

    _ = ::yew::html! {
        for Pair { value1, value2 } in ::std::iter::Iterator::map(0 .. 10, |value2| Pair { value1: "Yew", value2 }) {
            <span>{value1}</span>
            <span>{value2}</span>
        }
    };

    fn rand_number() -> ::std::primitive::u32 {
        4 // chosen by fair dice roll. guaranteed to be random.
    }

    _ = ::yew::html!{
        for _ in 0..5 {
            <div>
                {{
                    loop {
                        let a = rand_number();
                        if a % 2 == 0 {
                            break a;
                        }
                    }
                }}
            </div>
        }
    }
}
