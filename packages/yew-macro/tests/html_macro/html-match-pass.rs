#![no_implicit_prelude]

// Shadow primitives
#[allow(non_camel_case_types)]
pub struct bool;
#[allow(non_camel_case_types)]
pub struct char;
#[allow(non_camel_case_types)]
pub struct f32;
#[allow(non_camel_case_types)]
pub struct f64;
#[allow(non_camel_case_types)]
pub struct i128;
#[allow(non_camel_case_types)]
pub struct i16;
#[allow(non_camel_case_types)]
pub struct i32;
#[allow(non_camel_case_types)]
pub struct i64;
#[allow(non_camel_case_types)]
pub struct i8;
#[allow(non_camel_case_types)]
pub struct isize;
#[allow(non_camel_case_types)]
pub struct str;
#[allow(non_camel_case_types)]
pub struct u128;
#[allow(non_camel_case_types)]
pub struct u16;
#[allow(non_camel_case_types)]
pub struct u32;
#[allow(non_camel_case_types)]
pub struct u64;
#[allow(non_camel_case_types)]
pub struct u8;
#[allow(non_camel_case_types)]
pub struct usize;

fn main() {
    let status: ::std::primitive::u8 = 1;

    // Simple match with two arms
    _ = ::yew::html! {
        match status {
            0 => { <span>{"loading"}</span> }
            _ => { <span>{"done"}</span> }
        }
    };

    // Match with if guard
    _ = ::yew::html! {
        match status {
            x if x > 5 => { <span>{"big"}</span> }
            _ => { <span>{"small"}</span> }
        }
    };

    // Match with OR-patterns
    _ = ::yew::html! {
        match status {
            0 | 1 => { <span>{"low"}</span> }
            _ => { <span>{"high"}</span> }
        }
    };

    // Match with pattern destructuring
    let pair: (::std::primitive::i32, ::std::primitive::i32) = (1, 2);
    _ = ::yew::html! {
        match pair {
            (0, _) => { <span>{"zero first"}</span> }
            (_, 0) => { <span>{"zero second"}</span> }
            _ => { <span>{"other"}</span> }
        }
    };

    // Match inside element children
    _ = ::yew::html! {
        <div>
            match status {
                0 => { <span>{"zero"}</span> }
                _ => { <span>{"nonzero"}</span> }
            }
        </div>
    };

    // Match at root
    _ = ::yew::html! {
        match status {
            0 => { <span>{"a"}</span> }
            _ => { <span>{"b"}</span> }
        }
    };

    // Match with trailing commas
    _ = ::yew::html! {
        match status {
            0 => { <span>{"zero"}</span> },
            _ => { <span>{"other"}</span> },
        }
    };

    // Match with empty arm body
    _ = ::yew::html! {
        match status {
            0 => {}
            _ => { <span>{"fallback"}</span> }
        }
    };

    // Match with multiple children in arm body
    _ = ::yew::html! {
        match status {
            0 => { <span>{"a"}</span><span>{"b"}</span> }
            _ => { <div>{"c"}</div> }
        }
    };

    // this fails. but the message is cryptic
    // _ = ::yew::html! {
    //     match status {
    //         0 => <span>{"a"}</span><span>{"b"}</span>
    //         _ => { <div>{"c"}</div> }
    //     }
    // };


    // no trailing comma needed.
    _ = ::yew::html! {
        match status {
            0 => <><span>{"a"}</span><span>{"b"}</span></>
            _ => <div>{"c"}</div>
        }
    };

    // no trailing comma needed.
    _ = ::yew::html! {
        match status {
            0 => <span>{"a"}</span>
            _ => <div>{"c"}</div>
        }
    };

    // Match with if-let style via Option
    let opt: ::std::option::Option<::std::primitive::i32> = ::std::option::Option::Some(42);
    _ = ::yew::html! {
        match opt {
            ::std::option::Option::Some(val) => { <span>{val}</span> }
            ::std::option::Option::None => { <span>{"none"}</span> }
        }
    };

    // Match alongside other elements
    _ = ::yew::html! {
        <div>{"before"}</div>
        match status {
            0 => { <span>{"zero"}</span> }
            _ => { <span>{"other"}</span> }
        }
        <div>{"after"}</div>
    };

    // Unbraced single element arm
    _ = ::yew::html! {
        match status {
            0 => <span>{"zero"}</span>,
            _ => <div>{"other"}</div>,
        }
    };

    // Mixed braced and unbraced arms
    _ = ::yew::html! {
        match status {
            0 => <span>{"zero"}</span>,
            _ => { <div>{"other"}</div><div>{"more"}</div> }
        }
    };

    // Unbraced self-closing component-style element
    _ = ::yew::html! {
        match status {
            0 => <br/>,
            _ => <hr/>,
        }
    };

    // Let bindings in braced arm body
    _ = ::yew::html! {
        match opt {
            ::std::option::Option::Some(val) => {
                let doubled = val * 2;
                <span>{doubled}</span>
            }
            ::std::option::Option::None => { <span>{"none"}</span> }
        }
    };

    // Multiple let bindings in arm body
    _ = ::yew::html! {
        match opt {
            ::std::option::Option::Some(val) => {
                let doubled = val * 2;
                let label = "value";
                <span>{label}</span>
                <span>{doubled}</span>
            }
            ::std::option::Option::None => { <span>{"none"}</span> }
        }
    };

    // Let binding with type annotation in arm body
    _ = ::yew::html! {
        match status {
            0 => {
                let x: ::std::primitive::i32 = 42;
                <span>{x}</span>
            }
            _ => <span>{"other"}</span>,
        }
    };

    // For loop in unbraced match arm (VList -> VNode via Into)
    _ = ::yew::html! {
        match status {
            0 => for _ in 0..3_u8 { <span>{"repeated"}</span> },
            _ => <span>{"other"}</span>,
        }
    };

    // If expression in unbraced match arm
    _ = ::yew::html! {
        match status {
            0 => if status == 0 { <span>{"zero"}</span> },
            _ => <span>{"other"}</span>,
        }
    };

    // Nested match in unbraced match arm
    _ = ::yew::html! {
        match status {
            0 => match status {
                0 => { <span>{"inner-zero"}</span> }
                _ => { <span>{"inner-other"}</span> }
            },
            _ => <span>{"outer-other"}</span>,
        }
    };

    // Unbraced bare string literal
    _ = ::yew::html! {
        match status {
            0 => "zero",
            _ => "other",
        }
    };

    // Unbraced bare expression (format! macro)
    _ = ::yew::html! {
        match status {
            0 => ::std::format!("status is {}", status),
            _ => "other",
        }
    };

    // Bare string literal in braced match arm body
    _ = ::yew::html! {
        match status {
            0 => { "zero" }
            _ => { "other" }
        }
    };

    // Bare numeric literal in braced match arm body
    _ = ::yew::html! {
        match status {
            0 => { 42 }
            _ => { 0 }
        }
    };

    // Bare expression (format! macro) in braced match arm body
    _ = ::yew::html! {
        match status {
            0 => { ::std::format!("status is {}", status) }
            _ => { "other" }
        }
    };

    // Bare literal alongside HTML elements in match arm body
    _ = ::yew::html! {
        match status {
            0 => {
                "prefix "
                <span>{"content"}</span>
            }
            _ => { "other" }
        }
    };

    // Bare variable reference in braced match arm body
    let msg = "hello";
    _ = ::yew::html! {
        match status {
            0 => { msg }
            _ => { "other" }
        }
    };

    // Unbraced bare variable reference
    _ = ::yew::html! {
        match status {
            0 => msg,
            _ => "other",
        }
    };
}
