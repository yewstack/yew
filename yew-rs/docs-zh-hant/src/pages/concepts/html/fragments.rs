pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            code("html!"),
            " 巨集總是需要一個根節點。為了繞過這個限制，您可以使用一個\"空標籤\"（也稱為\"\
             fragments\"）。",
        ],
        tabs![
            "Valid",
            tab![
                "Valid",
                "Valid",
                code_block(
                    "rust",
                    r#"use yew::prelude::*;

html! {
    <>
        <div></div>
        <p></p>
    </>
};"#,
                ),
            ],
            tab![
                "Invalid",
                "Invalid",
                code_block(
                    "rust",
                    r#"use yew::prelude::*;

// 錯誤：只允許一個根 HTML 元素

html! {
    <div></div>
    <p></p>
};"#,
                ),
            ],
        ],
    ])
}

crate::doc_page!(
    "空標籤 (Fragments)",
    "/zh-Hant/docs/concepts/html/fragments",
    page_content()
);
