#![recursion_limit = "128"]

use yew::prelude::*;

struct Unimplemented;

fn compile_fail() {
    html! { <Unimplemented /> };
}

fn main() {}
