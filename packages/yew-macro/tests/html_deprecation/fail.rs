use yew::prelude::*;

fn root_level_fragment() {
    let _ = html! { <></> };
    let _ = html! { <><div/></> };
    let _ = html! {
        <>
            <span>{ "a" }</span>
            <span>{ "b" }</span>
        </>
    };
}

fn root_level_fragment_with_keyed_children() {
    let _ = html! {
        <>
            <p key="header">{"Menu Header"}</p>
        </>
    };
}

fn fragment_in_for_body() {
    let _ = html! {
        for _ in 0..3 {
            <><span>{"a"}</span><span>{"b"}</span></>
        }
    };
}

fn fragment_in_if_body() {
    let _ = html! { if true { <><div/><div/></> } };
}

fn fragment_in_else_body() {
    let _ = html! { if true { <div/> } else { <><span/><span/></> } };
}

fn fragment_in_match_arm() {
    let _ = html! {
        match 1 {
            1 => { <><h1>{"Hello"}</h1><p>{"World"}</p></> }
            _ => <h1>{"Goodbye"}</h1>,
        }
    };
}

fn html_in_unbraced_match_arm() {
    let _ = html! {
        match 1 {
            1 => html! { <h1>{"Hello"}</h1> },
            _ => <h1>{"Goodbye"}</h1>,
        }
    };
}

fn html_in_braced_match_arm() {
    let _ = html! {
        match 1 {
            1 => { let a = 1; html! { <h1>{a}</h1> } },
            _ => <h1>{"Goodbye"}</h1>,
        }
    };
}

fn match_in_block_with_html_arms() {
    let status: u8 = 0;
    let _ = html! {
        <div>{
            match status {
                0 => html! { <span>{"loading"}</span> },
                _ => html! { <span>{"done"}</span> },
            }
        }</div>
    };
}

fn nested_block_with_html_tail() {
    let item = "test";
    let _ = html! {
        <div>{{ let processed = item.to_uppercase(); html! { <span>{processed}</span> } }}</div>
    };
}

fn if_else_block_with_html_branches() {
    let cond = true;
    let _ = html! {
        <div>{ if cond { html! { <span>{"yes"}</span> } } else { html! { <span>{"no"}</span> } } }</div>
    };
}

fn main() {
    compile_error!("This macro call exists to deliberately fail the compilation of the test so we can verify output of deprecation lints");
}
