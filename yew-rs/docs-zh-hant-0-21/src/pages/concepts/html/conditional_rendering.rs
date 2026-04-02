crate::doc_page!(
    "條件渲染",
    "/zh-Hant/docs/concepts/html/conditional-rendering",
    Content::new(vec![
        h2!["If 區塊"],
        p![
            "要有條件地渲染一些標記，我們將其包裝在 ",
            code("if"),
            " 區塊中：",
        ],
        tabs![
            "if",
            tab![
                "if",
                "if",
                code_block(
                    "rust",
                    r#"use yew::prelude::*;

html! {
    if true {
        <p>{ "True case" }</p>
    }
};"#,
                ),
            ],
            tab![
                "if - else",
                "if - else",
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
};"#,
                ),
            ],
            tab![
                "if let",
                "if let",
                code_block(
                    "rust",
                    r#"use yew::prelude::*;
let some_text = Some("text");

html! {
    if let Some(text) = some_text {
        <p>{ text }</p>
    }
};"#,
                ),
            ],
            tab![
                "if let else",
                "if let else",
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
};"#,
                ),
            ],
        ],
    ])
    .with_description("Rendering nodes conditionally in html!")
);
