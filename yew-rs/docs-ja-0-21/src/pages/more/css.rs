crate::doc_page!(
    "CSS",
    "/ja/docs/more/css",
    Content::new(vec![
        p![text("<TODO>")],
        p![
            text("統合的なCSSサポートについての提案はこちらにあります: "),
            link!(
                "https://github.com/yewstack/yew/issues/533",
                text("https://github.com/yewstack/yew/issues/533")
            ),
        ],
        h2![text("スタイルフレームワーク:")],
        p![text(
            "今のところ、コミュニティメンバーは以下のスタイルフレームワークを開発しています。"
        )],
        ul![
            li![
                link!("https://github.com/spielrs/yew_styles", text("yew_styles")),
                text(" - JavaScriptに依存しないYewのスタイルフレームワーク"),
            ],
            li![
                link!("https://github.com/Follpvosten/yew-mdc", text("yew-mdc")),
                text(" - マテリアルデザインのコンポーネント"),
            ],
            li![
                link!(
                    "https://github.com/AlephAlpha/muicss-yew",
                    text("muicss-yew")
                ),
                text(" - MUIのCSSコンポーネント"),
            ],
            li![
                link!("https://github.com/yewstack/yewtify", text("Yewtify")),
                text(" - YewでVuetifyフレームワークで提供されている機能の実装"),
            ],
        ],
    ])
);
