pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            code("html!"),
            " マクロは常にルートノードを必要とします。この制限を回避するために、\
             「空のタグ」（または fragments）を使用できます。",
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

// エラー: ルート HTML 要素は1つだけ許可されます

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
    "空のタグ (Fragments)",
    "/ja/docs/concepts/html/fragments",
    page_content()
);
