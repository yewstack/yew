#![deny(unused_must_use)]

use yew::prelude::*;

fn not_a_hook() {
    use_effect_with((), |_| {});
}

fn main() {}
