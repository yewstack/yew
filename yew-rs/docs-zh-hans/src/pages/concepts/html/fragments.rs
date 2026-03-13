pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![
            code("html!"),
            text(" 宏总是需要一个根节点。为了绕过这个限制，"),
            text("您可以使用一个\"空标签\"（也称为\"fragments\"）。"),
        ]),
        tabs(
            "Valid",
            vec![
                tab(
                    "Valid",
                    "Valid",
                    vec![code_block(
                        "rust",
                        r#"use yew::prelude::*;

html! {
    <>
        <div></div>
        <p></p>
    </>
};"#,
                    )],
                ),
                tab(
                    "Invalid",
                    "Invalid",
                    vec![code_block(
                        "rust",
                        r#"use yew::prelude::*;

// 错误：只允许一个根 HTML 元素

html! {
    <div></div>
    <p></p>
};"#,
                    )],
                ),
            ],
        ),
    ])
}

crate::doc_page!(
    "空标签 (Fragments)",
    "/zh-Hans/docs/concepts/html/fragments",
    page_content()
);
