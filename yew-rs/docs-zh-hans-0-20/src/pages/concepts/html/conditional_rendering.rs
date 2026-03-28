crate::doc_page!(
    "Conditional Rendering",
    "/zh-Hans/docs/concepts/html/conditional-rendering",
    Content::new(vec![
        h2!["If blocks"],
        p![
            "To conditionally render some markup, we wrap it in an ",
            code("if"),
            " block:",
        ],
        p![code("if"), ":"],
        code_block(
            "rust",
            r#"use yew::prelude::*;

html! {
    if true {
        <p>{ "True case" }</p>
    }
};"#
        ),
        p![code("if - else"), ":"],
        code_block(
            "rust",
            r#"use yew::prelude::*;
let some_condition = true;

html! {
    if some_condition {
        <p>{ "True case" }</p>
    } else {
        <p>{ "False case" }</p>
    }
};"#
        ),
        p![code("if let"), ":"],
        code_block(
            "rust",
            r#"use yew::prelude::*;
let some_text = Some("text");

html! {
    if let Some(text) = some_text {
        <p>{ text }</p>
    }
};"#
        ),
        p![code("if let else"), ":"],
        code_block(
            "rust",
            r#"use yew::prelude::*;
let some_text = Some("text");

html! {
    if let Some(text) = some_text {
        <p>{ text }</p>
    } else {
        <p>{ "False case" }</p>
    }
};"#
        ),
    ])
    .with_description("Rendering nodes conditionally in html!")
);
