#![no_implicit_prelude]

#[::yew::prelude::hook]
fn use_str_ref(f: &::std::primitive::str) -> &::std::primitive::str {
    f
}

fn main() {}
