#![no_implicit_prelude]

#[::yew::prelude::hook]
fn use_deref_as_u32() -> impl ::std::ops::Deref<Target = ::std::primitive::u32> {
    ::std::rc::Rc::new(0)
}

fn main() {}
