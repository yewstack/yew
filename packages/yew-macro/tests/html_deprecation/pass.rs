#![deny(deprecated)]

use yew::prelude::*;

#[allow(deprecated)]
fn silence_unnecessary_root_fragment() {
    let _ = html! { <><div/></> };
}

#[allow(deprecated)]
fn silence_unnecessary_if_fragment() {
    let _ = html! { if true { <><div/><div/></> } };
}

#[allow(deprecated)]
fn silence_unnecessary_else_fragment() {
    let _ = html! { if true { <div/> } else { <><span/><span/></> } };
}

#[allow(deprecated)]
fn silence_unnecessary_match_fragment() {
    let _ = html! {
        match 1 {
            1 => { <><h1>{"Hello"}</h1><p>{"World"}</p></> }
            _ => <h1>{"Goodbye"}</h1>,
        }
    };
}

#[allow(deprecated)]
fn silence_html_in_match_arm() {
    let _ = html! {
        match 1 {
            1 => html! { <h1>{"Hello"}</h1> },
            _ => <h1>{"Goodbye"}</h1>,
        }
    };
}

#[allow(deprecated)]
fn silence_html_in_block_match() {
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

#[allow(deprecated)]
fn silence_html_in_nested_block() {
    let item = "test";
    let _ = html! {
        <div>{{ let processed = item.to_uppercase(); html! { <span>{processed}</span> } }}</div>
    };
}

#[allow(deprecated)]
fn silence_keyed_children_fragment() {
    let _ = html! {
        if true {
            <>
                <div key="a"/>
                <div key="b"/>
            </>
        }
    };
}

#[allow(deprecated)]
fn silence_for_loop_fragment() {
    let _ = html! {
        for _ in 0..3 {
            <><span>{"a"}</span><span>{"b"}</span></>
        }
    };
}

fn keyed_fragment_no_warning() {
    let _ = html! {
        if true {
            <key="outer">
                <div/>
                <div/>
            </>
        }
    };
}

fn main() {}
