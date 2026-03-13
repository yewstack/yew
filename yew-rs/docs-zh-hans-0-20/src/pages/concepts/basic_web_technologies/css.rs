crate::doc_page!(
    "CSS with classes!",
    "/zh-Hans/docs/concepts/basic-web-technologies/css",
    Content::new(vec![
        p(vec![
            text(
                "Yew does not natively provide a css in rust solution, but helps with styling by \
                 providing programmatic ways to interact with the html "
            ),
            code("class"),
            text(" attribute."),
        ]),
        h2(vec![text("Classes")]),
        p(vec![
            text("The "),
            code("classes!"),
            text(" macro and associated "),
            code("Classes"),
            text(" struct simplify the use of HTML classes:"),
        ]),
        h3(vec![text("Literal")]),
        code_block(
            "rust",
            r#"use yew::{classes, html};

html! {
  <div class={classes!("container")}></div>
};"#
        ),
        h3(vec![text("Multiple")]),
        code_block(
            "rust",
            r#"use yew::{classes, html};

html! {
  <div class={classes!("class-1", "class-2")}></div>
};"#
        ),
        h3(vec![text("String")]),
        code_block(
            "rust",
            r#"use yew::{classes, html};

html! {
  <div class={classes!(String::from("class-1 class-2"))}></div>
};"#
        ),
        h3(vec![text("Optional")]),
        code_block(
            "rust",
            r#"use yew::{classes, html};

html! {
  <div class={classes!(Some("class"))} />
};"#
        ),
        h3(vec![text("Vector")]),
        code_block(
            "rust",
            r#"use yew::{classes, html};

html! {
  <div class={classes!(vec!["class-1", "class-2"])}></div>
};"#
        ),
        h3(vec![text("Slice")]),
        code_block(
            "rust",
            r#"use yew::{classes, html};

html! {
  <div class={classes!(["class-1", "class-2"].as_ref())}></div>
};"#
        ),
        p(vec![
            text("We will expand upon this concept in "),
            link("/docs/0.20/more/css", vec![text("more CSS")]),
            text("."),
        ]),
        h2(vec![text("Inline Styles")]),
        p(vec![
            text(
                "Currently Yew does not provide any special help with inline styles specified via \
                 the "
            ),
            code("styles"),
            text(" attribute, but you can use it like any other html attribute:"),
        ]),
        code_block(
            "rust",
            r#"use yew::{classes, html};

html! {
  <div style="color: red;"></div>
};"#
        ),
        p(vec![
            text("We will expand upon this concept in "),
            link("/docs/0.20/more/css", vec![text("more CSS")]),
            text("."),
        ]),
    ])
);
