pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            text("Yew 并没有提供原生的 CSS-in-Rust 解决方案，但通过提供编程方式与 HTML "),
            code("class"),
            text(" 属性交互的方式来辅助样式。"),
        ],
        h2![text("`classes!` 宏")],
        p![
            code("classes!"),
            text(" 宏和相关的 "),
            code("Classes"),
            text(" 结构简化了 HTML 类的使用："),
        ],
        tabs![
            "Literal",
            tab![
                "Literal",
                "Literal",
                code_block(
                    "rust",
                    r#"use yew::{classes, html};

html! {
  <div class={classes!("container")}></div>
};"#,
                ),
            ],
            tab![
                "Multiple",
                "Multiple",
                code_block(
                    "rust",
                    r#"use yew::{classes, html};

html! {
  <div class={classes!("class-1", "class-2")}></div>
};"#,
                ),
            ],
            tab![
                "String",
                "String",
                code_block(
                    "rust",
                    r#"use yew::{classes, html};

html! {
  <div class={classes!(String::from("class-1 class-2"))}></div>
};"#,
                ),
            ],
            tab![
                "Optional",
                "Optional",
                code_block(
                    "rust",
                    r#"use yew::{classes, html};

html! {
  <div class={classes!(Some("class"))} />
};"#,
                ),
            ],
            tab![
                "Vector",
                "Vector",
                code_block(
                    "rust",
                    r#"use yew::{classes, html};

html! {
  <div class={classes!(vec!["class-1", "class-2"])}></div>
};"#,
                ),
            ],
            tab![
                "Slice",
                "Slice",
                code_block(
                    "rust",
                    r#"use yew::{classes, html};

html! {
  <div class={classes!(["class-1", "class-2"].as_ref())}></div>
};"#,
                ),
            ],
        ],
        p![
            text("更多 CSS 相关的内容请参见"),
            link!("/zh-Hans/docs/more/css", text("这个文档")),
            text("。"),
        ],
        h2![text("内联样式")],
        p![
            text("目前 Yew 并没有提供特殊的辅助工具来处理通过 "),
            code("style"),
            text(" 属性指定的内联样式，但你可以像处理其他 HTML 属性一样处理它："),
        ],
        code_block(
            "rust",
            r#"use yew::{classes, html};

html! {
  <div style="color: red;"></div>
};"#,
        ),
        p![
            text("更多 CSS 相关的内容请参见"),
            link!("/zh-Hans/docs/more/css", text("这个文档")),
            text("。"),
        ],
    ])
}

crate::doc_page!(
    "使用 classes! 宏处理 CSS 类",
    "/zh-Hans/docs/concepts/basic-web-technologies/css",
    page_content()
);
