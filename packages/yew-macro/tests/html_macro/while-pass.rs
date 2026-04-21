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
    // Basic while with counter bumped via a let-bound post-increment block
    _ = {
        let mut i: ::std::primitive::i32 = 0;
        ::yew::html! {
            while i < 5 {
                let current = { let c = i; i += 1; c };
                <span>{current}</span>
            }
        }
    };

    // while let destructuring an iterator
    _ = {
        let mut it = ::std::iter::IntoIterator::into_iter(0..5);
        ::yew::html! {
            while let ::std::option::Option::Some(v) = ::std::iter::Iterator::next(&mut it) {
                <span>{v}</span>
            }
        }
    };

    // while let with pattern destructuring
    _ = {
        let mut it = ::std::iter::IntoIterator::into_iter([(1, "a"), (2, "b")]);
        ::yew::html! {
            while let ::std::option::Option::Some((n, s)) = ::std::iter::Iterator::next(&mut it) {
                <span>{n}</span>
                <span>{s}</span>
            }
        }
    };

    // Multiple let bindings before nodes
    _ = {
        let mut i: ::std::primitive::i32 = 0;
        ::yew::html! {
            while i < 5 {
                let current = { let c = i; i += 1; c };
                let doubled = current * 2;
                let label = "item";
                <span>{label}</span>
                <span>{doubled}</span>
            }
        }
    };

    // Let binding with explicit type annotation
    _ = {
        let mut i: ::std::primitive::i32 = 0;
        ::yew::html! {
            while i < 5 {
                let x: ::std::primitive::i32 = { let c = i * 3; i += 1; c };
                <span>{x}</span>
            }
        }
    };

    // break in while
    _ = {
        let mut i: ::std::primitive::i32 = 0;
        ::yew::html! {
            while i < 100 {
                let current = { let c = i; i += 1; c };
                if current > 5 {
                    break
                }
                <span>{current}</span>
            }
        }
    };

    // continue in while
    _ = {
        let mut i: ::std::primitive::i32 = 0;
        ::yew::html! {
            while i < 10 {
                let current = { let c = i; i += 1; c };
                if current % 2 == 0 {
                    continue
                }
                <span>{current}</span>
            }
        }
    };

    // while-let with break and continue
    _ = {
        let mut it = ::std::iter::IntoIterator::into_iter(0..10);
        ::yew::html! {
            while let ::std::option::Option::Some(v) = ::std::iter::Iterator::next(&mut it) {
                if v >= 6 {
                    break
                }
                if v % 2 == 0 {
                    continue
                }
                <span>{v}</span>
            }
        }
    };

    // Nested for inside while
    _ = {
        let mut i: ::std::primitive::i32 = 0;
        ::yew::html! {
            while i < 3 {
                let row = { let c = i; i += 1; c };
                for col in 0..3 {
                    <span>{row}</span>
                    <span>{col}</span>
                }
            }
        }
    };
}
