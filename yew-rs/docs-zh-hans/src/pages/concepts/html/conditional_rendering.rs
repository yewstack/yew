pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["If 块"],
        p![
            "要有条件地渲染一些标记，我们将其包装在 ",
            code("if"),
            " 块中：",
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
}

crate::doc_page!(
    "条件渲染",
    "/zh-Hans/docs/concepts/html/conditional-rendering",
    page_content()
);
