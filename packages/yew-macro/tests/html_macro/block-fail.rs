use yew::prelude::*;

fn compile_fail() {

    let not_tree = || ();
    html! {
        <div>{ not_tree() }</div>
    };
    html! {
        <div>{ for (0..3).map(|_| not_tree()) }</div>
    };
}

fn deprecated_match_with_html() {
    let status: u8 = 0;

    // Old pattern: match inside block with html! arms
    html! {
        <div>{
            match status {
                0 => html! { <span>{"loading"}</span> },
                _ => html! { <span>{"done"}</span> },
            }
        }</div>
    };
}

fn deprecated_block_with_html() {
    let item = "test";

    // Old pattern: nested block with html! as tail expression
    html! {
        <div>{{ let processed = item.to_uppercase(); html! { <span>{processed}</span> } }}</div>
    };
}

fn main() {}
