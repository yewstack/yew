use yew::prelude::*;

fn compile_fail() {
    html! { <> };
    html! { </> };
    html! { <><> };
    html! { </></> };
    html! { <><></> };
    html! { <></><></> };
    html! { <>invalid</> };
    html! { <key=></> };
    html! { <key="key".to_string()></key> };

    html! { <key="first key" key="second key" /> };
    html! { <some_attr="test"></> };

    html! { <key?=None></> };
}

fn main() {}
