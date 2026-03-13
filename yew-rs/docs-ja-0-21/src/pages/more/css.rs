crate::doc_page!(
    "CSS",
    "/ja/docs/more/css",
    Content::new(vec![
        p(vec![text("<TODO>")]),
        p(vec![
            text("統合的なCSSサポートについての提案はこちらにあります: "),
            link(
                "https://github.com/yewstack/yew/issues/533",
                vec![text("https://github.com/yewstack/yew/issues/533")]
            ),
        ]),
        h2(vec![text("スタイルフレームワーク:")]),
        p(vec![text(
            "今のところ、コミュニティメンバーは以下のスタイルフレームワークを開発しています。"
        )]),
        ul(vec![
            li(vec![
                link(
                    "https://github.com/spielrs/yew_styles",
                    vec![text("yew_styles")]
                ),
                text(" - JavaScriptに依存しないYewのスタイルフレームワーク"),
            ]),
            li(vec![
                link(
                    "https://github.com/Follpvosten/yew-mdc",
                    vec![text("yew-mdc")]
                ),
                text(" - マテリアルデザインのコンポーネント"),
            ]),
            li(vec![
                link(
                    "https://github.com/AlephAlpha/muicss-yew",
                    vec![text("muicss-yew")]
                ),
                text(" - MUIのCSSコンポーネント"),
            ]),
            li(vec![
                link("https://github.com/yewstack/yewtify", vec![text("Yewtify")]),
                text(" - YewでVuetifyフレームワークで提供されている機能の実装"),
            ]),
        ]),
    ])
);
