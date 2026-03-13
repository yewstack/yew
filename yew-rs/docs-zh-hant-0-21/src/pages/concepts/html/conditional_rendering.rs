crate::doc_page!(
    "條件渲染",
    "/zh-Hant/docs/concepts/html/conditional-rendering",
    Content::new(vec![
        h2(vec![text("If 區塊")]),
        p(vec![
            text("要有條件地渲染一些標記，我們將其包裝在 "),
            code("if"),
            text(" 區塊中："),
        ]),
        tabs(
            "if",
            vec![
                tab(
                    "if",
                    "if",
                    vec![code_block(
                        "rust",
                        r#"use yew::prelude::*;

html! {
    if true {
        <p>{ "True case" }</p>
    }
};"#,
                    )],
                ),
                tab(
                    "if - else",
                    "if - else",
                    vec![code_block(
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
                    )],
                ),
                tab(
                    "if let",
                    "if let",
                    vec![code_block(
                        "rust",
                        r#"use yew::prelude::*;
let some_text = Some("text");

html! {
    if let Some(text) = some_text {
        <p>{ text }</p>
    }
};"#,
                    )],
                ),
                tab(
                    "if let else",
                    "if let else",
                    vec![code_block(
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
                    )],
                ),
            ],
        ),
    ])
);
