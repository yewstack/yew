crate::doc_page!(
    "空標籤 (Fragments)",
    "/zh-Hant/docs/concepts/html/fragments",
    Content::new(vec![
        p![
            code("html!"),
            text(
                " 巨集總是需要一個根節點。為了繞過這個限制，您可以使用一個\"空標籤\"（也稱為\"\
                 fragments\"）。",
            ),
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
                code_block_compile_fail(
                    "rust",
                    r#"use yew::prelude::*;

// error: only one root html element allowed

html! {
    <div></div>
    <p></p>
};"#,
                ),
            ],
        ],
    ])
);
