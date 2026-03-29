crate::doc_page!(
    "CSS with classes!",
    "/ja/docs/concepts/basic-web-technologies/css",
    Content::new(vec![
        p![
            "Yew does not natively provide a CSS-in-Rust solution but helps with styling by \
             providing programmatic ways to interact with the HTML ",
            code("class"),
            " attribute.",
        ],
        h2!["Classes"],
        p![
            "The ",
            code("classes!"),
            " macro and associated ",
            code("Classes"),
            " struct simplify the use of HTML classes:",
        ],
        h3!["Literal"],
        code_block(
            "rust",
            "use yew::{classes, html};

html! {
  <div class={classes!(\"container\")}></div>
};"
        ),
        h3!["Multiple"],
        code_block(
            "rust",
            "use yew::{classes, html};

html! {
  <div class={classes!(\"class-1\", \"class-2\")}></div>
};"
        ),
        h3!["String"],
        code_block(
            "rust",
            "use yew::{classes, html};

html! {
  <div class={classes!(String::from(\"class-1 class-2\"))}></div>
};"
        ),
        h3!["Optional"],
        code_block(
            "rust",
            "use yew::{classes, html};

html! {
  <div class={classes!(Some(\"class\"))} />
};"
        ),
        h3!["Vector"],
        code_block(
            "rust",
            "use yew::{classes, html};

html! {
  <div class={classes!(vec![\"class-1\", \"class-2\"])}></div>
};"
        ),
        h3!["Slice"],
        code_block(
            "rust",
            "use yew::{classes, html};

html! {
  <div class={classes!([\"class-1\", \"class-2\"].as_ref())}></div>
};"
        ),
        p![
            "We will expand upon this concept in ",
            doc_link![crate::pages::more::css, "more CSS"],
            ".",
        ],
        h2!["Inline Styles"],
        p![
            "Currently Yew does not provide any special help with inline styles specified via the ",
            code("styles"),
            " attribute, but you can use it like any other HTML attribute:",
        ],
        code_block(
            "rust",
            "use yew::{classes, html};

html! {
  <div style=\"color: red;\"></div>
};"
        ),
        p![
            "We will expand upon this concept in ",
            doc_link![crate::pages::more::css, "more CSS"],
            ".",
        ],
    ])
    .with_description("A handy macro to handle classes")
);
