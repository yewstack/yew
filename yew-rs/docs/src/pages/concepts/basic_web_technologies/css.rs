pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            text(
                "Yew does not natively provide a CSS-in-Rust solution but helps with styling by \
                 providing programmatic ways to interact with the HTML ",
            ),
            code("class"),
            text(" attribute."),
        ],
        h2![code("classes!"), text(" macro")],
        p![
            text("The "),
            code("classes!"),
            text(" macro and associated "),
            code("Classes"),
            text(" struct simplify the use of HTML classes:"),
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
            text("We will expand upon this concept in "),
            link!("/docs/more/css", text("more CSS")),
            text("."),
        ],
        h2![text("Inline Styles")],
        p![
            text(
                "Currently Yew does not provide any special help with inline styles specified via \
                 the ",
            ),
            code("style"),
            text(" attribute, but you can use it like any other HTML attribute:"),
        ],
        code_block(
            "rust",
            "use yew::{classes, html};

html! {
  <div style=\"color: red;\"></div>
};",
        ),
        p![
            text("We will expand upon this concept in "),
            link!("/docs/more/css", text("more CSS")),
            text("."),
        ],
    ])
}

crate::doc_page!(
    "CSS with classes!",
    "/docs/concepts/basic-web-technologies/css",
    page_content()
);
