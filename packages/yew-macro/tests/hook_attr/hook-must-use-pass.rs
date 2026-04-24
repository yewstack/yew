#![deny(unused_must_use)]

use yew::prelude::*;

#[hook]
fn use_my_effect() {
    use_effect_with((), |_| {});
}

#[component]
fn Comp() -> Html {
    use_effect_with((), |_| {});
    use_my_effect();
    html! {}
}

fn main() {}
