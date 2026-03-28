pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["If blocks"],
        p![
            "To conditionally render some markup, we wrap it in an ",
            code("if"),
            " block:",
        ],
        tabs!(
            "if",
            tab!(
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
            ),
            tab!(
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
            ),
            tab!(
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
            ),
            tab!(
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
            ),
        ),
    ])
    .with_description("Rendering nodes conditionally in html!")
}

crate::doc_page!(
    "Conditional Rendering",
    "/docs/concepts/html/conditional-rendering",
    page_content()
);
