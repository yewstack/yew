// we need to re-test the macro hygiene here as it uses a different implementation for impl traits.
#![no_implicit_prelude]

#[::yew::prelude::hook]
fn use_some_string(a: impl ::std::convert::Into<::std::string::String>) -> ::std::string::String {
    a.into()
}

#[::yew::prelude::hook]
fn use_impl_fn<T, U>(_callback: impl ::std::prelude::rust_2021::Fn(&T) -> &U) {}

fn main() {}
