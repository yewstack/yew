#![no_implicit_prelude]

use yew::prelude::*;
use yew_functional::{functional_component};

#[functional_component(Comp)]
fn comp() -> Html {
    html! {
        <p>
            { "Test" }
        </p>
    }
}


fn main() {}
