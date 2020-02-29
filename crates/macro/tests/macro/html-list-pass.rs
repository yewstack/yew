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
}

fn main() {}
