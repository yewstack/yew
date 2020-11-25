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

    let children = vec![
        html! { <span>{ "Hello" }</span> },
        html! { <span>{ "World" }</span> },
    ];
    html! { <>{children}</> };
}

fn main() {}
