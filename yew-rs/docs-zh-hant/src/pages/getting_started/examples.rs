pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "Yew 倉庫包含許多",
            link!(
                "https://github.com/yewstack/yew/tree/master/examples",
                "範例",
            ),
            "（維護狀態各異）。 我們建議瀏覽它們以了解如何使用框架的不同功能。 \
             我們也歡迎拉取請求和問題，以便在它們不可避免地被忽略並需要一些幫助時使用。",
        ],
        p![
            "有關更多詳細信息，包括示例列表，請參閱",
            link!(
                "https://github.com/yewstack/yew/tree/master/examples#yew-examples",
                "README",
            ),
            "。",
        ],
        admonition!(
            AdmonitionType::Note,
            None,
            p![
                "大多數範例都有一個可以在 https://examples.yew.rs/< example_name > \
                 找到的線上部署。\n在各自的子資料夾中的 README \
                 頁面上點擊它們的徽章以導航到線上演示。"
            ],
        ),
    ])
}

crate::doc_page!(
    "範例",
    "/zh-Hant/docs/getting-started/examples",
    page_content()
);
