#![recursion_limit = "128"]

use yew::prelude::*;

fn compile_fail() {
    html! { <String /> };
}

fn main() {}
