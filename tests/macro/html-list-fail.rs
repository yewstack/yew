use yew::prelude::*;

fn compile_fail() {
    html! { <> };
    html! { </> };
    html! { <><> };
    html! { </></> };
    html! { <><></> };
    html! { <></><></> };
    html! { <>invalid</> };
}

fn main() {}
