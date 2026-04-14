#![no_implicit_prelude]

fn compile_pass_lit() {
    _ = ::yew::html! { if true {} };
    _ = ::yew::html! { if true { <div/> } };
    _ = ::yew::html! { if true { <div/><div/> } };
    _ = ::yew::html! { if true { <div/><div/> } };
    _ = ::yew::html! { if true {} else {} };
    _ = ::yew::html! { if true {} else if true {} };
    _ = ::yew::html! { if true {} else if true {} else {} };
    _ = ::yew::html! { if let ::std::option::Option::Some(text) = ::std::option::Option::Some("text") { <span>{ text }</span> } };
    _ = ::yew::html! { <div/>if true {}<div/> };
    _ = ::yew::html! { <div>if true {}</div> };
}

fn compile_pass_expr() {
    let condition = true;

    _ = ::yew::html! { if condition {} };
    _ = ::yew::html! { if condition { <div/> } };
    _ = ::yew::html! { if condition { <div/><div/> } };
    _ = ::yew::html! { if condition { <div/><div/> } };
    _ = ::yew::html! { if condition {} else {} };
    _ = ::yew::html! { if condition {} else if condition {} };
    _ = ::yew::html! { if condition {} else if condition {} else {} };
    _ = ::yew::html! { if let ::std::option::Option::Some(text) = ::std::option::Option::Some("text") { <span>{ text }</span> } };
    _ = ::yew::html! { <div/>if condition {}<div/> };
    _ = ::yew::html! { <div>if condition {}</div> };
}

fn compile_pass_bare_nodes() {
    let bar = "bar_val";

    // Bare string literal in if body
    _ = ::yew::html! { if true { "hello" } };

    // Bare variable in if body
    _ = ::yew::html! { if true { bar } };

    // Bare expression in if body
    _ = ::yew::html! { if true { ::std::format!("bar: {}", bar) } };

    // Bare literal in else body
    _ = ::yew::html! { if true { "foo" } else { "bar" } };

    // Bare expression in else body
    _ = ::yew::html! { if true { "foo" } else { ::std::format!("bar: {}", bar) } };

    // Bare literal in else-if chain
    _ = ::yew::html! { if true { "a" } else if true { "b" } else { "c" } };
}

fn compile_pass_let_bindings() {
    let bar = "bar_val";

    // Let binding in if body
    _ = ::yew::html! {
        if true {
            let bar_pretty = ::std::format!("bar is: {}", bar);
            <span>{bar_pretty}</span>
        }
    };

    // Multiple let bindings
    _ = ::yew::html! {
        if true {
            let a = 1;
            let b = 2;
            <span>{a + b}</span>
        }
    };

    // Let binding in else body
    _ = ::yew::html! {
        if true {
            <span>{"yes"}</span>
        } else {
            let msg = "no";
            <span>{msg}</span>
        }
    };

    // Let binding with type annotation
    _ = ::yew::html! {
        if true {
            let x: ::std::primitive::i32 = 42;
            <span>{x}</span>
        }
    };

    // Let binding with bare literal body
    _ = ::yew::html! {
        if true {
            let msg = "computed";
            msg
        }
    };

    // Let binding with multiple children
    _ = ::yew::html! {
        if true {
            let bar_pretty = ::std::format!("bar is: {}", bar);
            <p>{bar_pretty}</p>
            <p>{"repeat:"}</p>
        }
    };

    // Let binding with multiple bare children (no fragment wrapper)
    _ = ::yew::html! {
        if true {
            let msg = "hello";
            <p>{msg}</p>
            <p>{"world"}</p>
        }
    };

    // Multiple children in else body
    _ = ::yew::html! {
        if true {
            <span>{"a"}</span>
        } else {
            <span>{"b"}</span>
            <span>{"c"}</span>
        }
    };

    // Let binding + multiple children in else body
    _ = ::yew::html! {
        if true {
            <span>{"yes"}</span>
        } else {
            let msg = "no";
            <span>{msg}</span>
            <span>{"really"}</span>
        }
    };

    // Multiple children in else-if body
    _ = ::yew::html! {
        if true {
            <span>{"a"}</span>
        } else if true {
            <span>{"b"}</span>
            <span>{"c"}</span>
        } else {
            <span>{"d"}</span>
        }
    };
}

fn main() {}
