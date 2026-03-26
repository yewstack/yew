pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "Yew does not natively provide a CSS-in-Rust solution but helps with styling by \
             providing programmatic ways to interact with the HTML ",
            code("class"),
            " attribute.",
        ],
        h2![code("classes!"), " macro"],
        p![
            "The ",
            code("classes!"),
            " macro and associated ",
            code("Classes"),
            " struct simplify the use of HTML classes:",
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
            "We will expand upon this concept in ",
            doc_link!(crate::pages::more::css, "more CSS"),
            ".",
        ],
        h2!["Inline Styles"],
        p![
            "Currently Yew does not provide any special help with inline styles specified via the ",
            code("style"),
            " attribute, but you can use it like any other HTML attribute:",
        ],
        code_block(
            "rust",
            "use yew::{classes, html};

html! {
  <div style=\"color: red;\"></div>
};",
        ),
        p![
            "We will expand upon this concept in ",
            doc_link!(crate::pages::more::css, "more CSS"),
            ".",
        ],
    ])
}

crate::doc_page!(
    "CSS with classes!",
    "/docs/concepts/basic-web-technologies/css",
    page_content()
);
