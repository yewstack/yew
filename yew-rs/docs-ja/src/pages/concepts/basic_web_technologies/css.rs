pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "Yew はネイティブの CSS-in-Rust ソリューションを提供していませんが、HTML の ",
            code("class"),
            " 属性とプログラム的に対話する方法を提供することでスタイルを支援します。",
        ],
        h2![code("classes!"), " マクロ"],
        p![
            code("classes!"),
            " マクロと関連する ",
            code("Classes"),
            " 構造体は、HTML クラスの使用を簡素化します：",
        ],
        tabs(
            "Literal",
            vec![
                tab(
                    "Literal",
                    "Literal",
                    vec![code_block(
                        "rust",
                        r#"use yew::{classes, html};

html! {
  <div class={classes!("container")}></div>
};"#,
                    )],
                ),
                tab(
                    "Multiple",
                    "Multiple",
                    vec![code_block(
                        "rust",
                        r#"use yew::{classes, html};

html! {
  <div class={classes!("class-1", "class-2")}></div>
};"#,
                    )],
                ),
                tab(
                    "String",
                    "String",
                    vec![code_block(
                        "rust",
                        r#"use yew::{classes, html};

html! {
  <div class={classes!(String::from("class-1 class-2"))}></div>
};"#,
                    )],
                ),
                tab(
                    "Optional",
                    "Optional",
                    vec![code_block(
                        "rust",
                        r#"use yew::{classes, html};

html! {
  <div class={classes!(Some("class"))} />
};"#,
                    )],
                ),
                tab(
                    "Vector",
                    "Vector",
                    vec![code_block(
                        "rust",
                        r#"use yew::{classes, html};

html! {
  <div class={classes!(vec!["class-1", "class-2"])}></div>
};"#,
                    )],
                ),
                tab(
                    "Slice",
                    "Slice",
                    vec![code_block(
                        "rust",
                        r#"use yew::{classes, html};

html! {
  <div class={classes!(["class-1", "class-2"].as_ref())}></div>
};"#,
                    )],
                ),
            ],
        ),
        p![
            "詳細な CSS に関する内容は",
            doc_link!(crate::pages::more::css, "こちらのドキュメント"),
            "をご覧ください。",
        ],
        h2!["インラインスタイル"],
        p![
            "現在、Yew は ",
            code("style"),
            " 属性を使用して指定されたインラインスタイルを処理するための特別な支援ツールを提供していませんが、他の HTML 属性と同様に処理することができます：",
        ],
        code_block(
            "rust",
            "use yew::{classes, html};

html! {
  <div style=\"color: red;\"></div>
};"
        ),
        p![
            "詳細な CSS に関する内容は",
            doc_link!(crate::pages::more::css, "こちらのドキュメント"),
            "をご覧ください。",
        ],
    ])
    .with_description("A handy macro to handle classes")
}

crate::doc_page!(
    "classes! マクロを使用して CSS クラスを処理する",
    "/ja/docs/concepts/basic-web-technologies/css",
    page_content()
);
