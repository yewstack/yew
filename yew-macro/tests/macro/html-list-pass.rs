use yew::prelude::*;

fn compile_pass() {
    html! {};
    html! { <></> };
    html! {
        <>
            <></>
            <></>
        </>
    };
    html! {
        <key="key".to_string()>
        </>
    };
}

fn main() {}
