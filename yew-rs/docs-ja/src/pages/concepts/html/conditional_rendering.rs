pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("If ブロック")]),
        p(vec![
            text("条件付きでマークアップをレンダリングするには、それを "),
            code("if"),
            text(" ブロックでラップします："),
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
}

crate::doc_page!(
    "条件レンダリング",
    "/ja/docs/concepts/html/conditional-rendering",
    page_content()
);
