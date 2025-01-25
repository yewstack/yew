use yew::{Children, Html};
use yew_macro::{function_component, html, mdx, mdx_style, Properties};

//  h3 used for testing styling
#[cfg(test)]
mdx_style!(h3: MyHeading3,);

#[derive(Properties, PartialEq)]
struct MyPProps {
    children: Children,
}

#[function_component]
fn MyP(props: &MyPProps) -> Html {
    html! {
        <p>{props.children.clone()}</p>
    }
}

#[test]
fn text() {
    assert_eq!(
        mdx! {
            r#"hi"#
        },
        html! {<><p>{"hi"}</p></>}
    );
}

#[test]
fn h1() {
    dbg_eq(
        mdx! {
            r#"# hi"#
        },
        html! {
            <><h1>{"hi"}</h1></>
        },
    );
}

#[test]
fn a() {
    dbg_eq(
        mdx! {
            r#"[this is a link](google.com)"#
        },
        html! {
            <><p><a href="google.com">{"this is a link"}</a></p></>
        },
    )
}

#[test]
fn nested() {
    dbg_eq(
        mdx! {
            r#"# Wow a [link](google.com) in a title"#
        },
        html! {
            <><h1>{"Wow a "}<a href="google.com">{"link"}</a>{" in a title"}</h1></>
        },
    )
}

#[test]
fn multiple() {
    dbg_eq(
        mdx! {
            r#"Some text [link](google.com)"#
        },
        html! {
            <>
            <p>
            {"Some text "}
            <a href="google.com">{"link"}</a>
            </p>
            </>
        },
    )
}

#[test]
fn multiline_text() {
    dbg_eq(
        mdx! { r#"this is some text that
        spans multiple lines"#
        },
        html! {<><p>{"this is some text that"}{" "}{"spans multiple lines"}</p></>},
    )
}

#[test]
fn multiline_link() {
    dbg_eq(
        mdx! { r#"[this is a
        multiline link wow](google.com)"#},
        html! {
            <>
            <p>
            <a href="google.com">{"this is a"} {" "} {"multiline link wow"}</a>
            </p>
            </>
        },
    )
}

#[test]
fn basic_code() {
    dbg_eq(
        mdx! {r#"here is some `inline code` ooo"#},
        html! {
            <>
            <p>
            {"here is some "}<code>{"inline code"}</code>{" ooo"}
            </p>
            </>
        },
    );
    dbg_eq(
        mdx! {r#"# header `inline code` ooo"#},
        html! {
            <>
            <h1>
                {"header "}<code>{"inline code"}</code>{" ooo"}
            </h1>
            </>
        },
    );
    dbg_eq(
        mdx! {r#"# header [link `inline code`](google.com) ooo"#},
        html! {
            <>
            <h1>
                {"header "}<a href="google.com">{"link "}<code>{"inline code"}</code></a>{" ooo"}
            </h1>
            </>
        },
    );
}

#[test]
fn list() {
    dbg_eq(
        mdx! {r#"
- one
- two
- three
"#},
        html! {
            <>
            <ul>
            <li>{"one"}</li>
            <li>{"two"}</li>
            <li>{"three"}</li>
            </ul>
            </>

        },
    )
}

#[test]
fn component() {
    dbg_eq(
        mdx! {r#"
# <TestComponent />
"#},
        html! {
            <>
            <h1>
                <TestComponent />
            </h1>
            </>
        },
    );
}

#[test]
fn quotes_escaped() {
    //  NOTE: if quotes aren't correctly esacped, this will panic as invalid
    //  syntax because of
    //  https://doc.rust-lang.org/edition-guide/rust-2021/reserving-syntax.html#summary
    mdx!(r#"i "like"quotes"" """"#);
}

fn dbg_eq<T: std::fmt::Debug>(a: T, b: T) {
    assert_eq!(format!("{a:?}"), format!("{b:?}"));
}

#[function_component]
fn TestComponent() -> Html {
    html! {
        <div class="test-component"></div>
    }
}

#[derive(PartialEq, Properties)]
struct MyHeading3Props {
    pub children: Children,
}
#[function_component]
fn MyHeading3(c: &MyHeading3Props) -> Html {
    html! {
        <h3>
        <strong>
        {c.children.clone()}
        </strong>
        </h3>
    }
}

#[test]
fn style_h3() {
    dbg_eq(
        mdx! {r#"### 123"#},
        html! {
            <><MyHeading3>{"123"}</MyHeading3></>
        },
    )
}
