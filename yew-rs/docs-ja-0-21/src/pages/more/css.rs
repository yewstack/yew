crate::doc_page!(
    "CSS",
    "/ja/docs/more/css",
    Content::new(vec![
        p!["<TODO>"],
        p![
            "統合的なCSSサポートについての提案はこちらにあります: ",
            link!(
                "https://github.com/yewstack/yew/issues/533",
                "https://github.com/yewstack/yew/issues/533"
            ),
        ],
        h2!["スタイルフレームワーク:"],
        p!["今のところ、コミュニティメンバーは以下のスタイルフレームワークを開発しています。"],
        ul![
            li![
                link!("https://github.com/spielrs/yew_styles", "yew_styles"),
                " - JavaScriptに依存しないYewのスタイルフレームワーク",
            ],
            li![
                link!("https://github.com/Follpvosten/yew-mdc", "yew-mdc"),
                " - マテリアルデザインのコンポーネント",
            ],
            li![
                link!("https://github.com/AlephAlpha/muicss-yew", "muicss-yew"),
                " - MUIのCSSコンポーネント",
            ],
            li![
                link!("https://github.com/yewstack/yewtify", "Yewtify"),
                " - YewでVuetifyフレームワークで提供されている機能の実装",
            ],
        ],
    ])
);
