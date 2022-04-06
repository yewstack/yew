#![no_implicit_prelude]

#[::yew::functional::hook]
fn use_as_is<'a>(input: &'a ()) -> &'a () {
    input
}

fn main() {}
