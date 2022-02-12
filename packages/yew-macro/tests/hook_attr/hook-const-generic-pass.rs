#![no_implicit_prelude]

#[::yew::prelude::hook]
fn use_a_const<const N: u32>() -> u32 {
    N
}

fn main() {}
