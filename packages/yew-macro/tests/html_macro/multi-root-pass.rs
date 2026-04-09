#![no_implicit_prelude]

fn main() {
    // multi-root with elements
    _ = ::yew::html! {
        <span>{ "a" }</span>
        <span>{ "b" }</span>
    };

    // multi-root with mixed literal and element
    _ = ::yew::html! {
        "prefix"
        <span>{ "b" }</span>
    };

    // multi-root with `if` expression and elements
    _ = ::yew::html! {
        <span>{ "a" }</span>
        if true {
            <span>{ "b" }</span>
        }
    };

    // multi-root with `for` expression and elements
    _ = ::yew::html! {
        <span>{ "header" }</span>
        for x in 0..3 {
            <span>{ x }</span>
        }
    };

    // multi-root with `match` expression
    let n: i32 = 0;
    _ = ::yew::html! {
        <span>{ "header" }</span>
        match n {
            0 => <span>{ "zero" }</span>,
            _ => <span>{ "other" }</span>,
        }
    };

    // multi-root in `html_nested!`
    let _nested: ::yew::virtual_dom::VNode = ::yew::html_nested! {
        <span>{ "1" }</span>
        <span>{ "2" }</span>
    };

    // empty html!
    _ = ::yew::html! {};

    // single-root still works
    _ = ::yew::html! { <div/> };

    // single bare literal still works
    _ = ::yew::html! { "single literal" };

    // multi-root of bare literals
    _ = ::yew::html! {
        "a"
        "b"
        "c"
    };
}
