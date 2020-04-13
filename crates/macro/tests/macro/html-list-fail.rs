use yew::prelude::*;

fn compile_fail() {
    html! { <> };
    html! { </> };
    html! { <><> };
    html! { </></> };
    html! { <><></> };
    html! { <></><></> };
    html! { <>invalid</> };
    html! { <key=></>}
    html! { <key="key".to_string()>invalid</key>}
}

fn main() {}
