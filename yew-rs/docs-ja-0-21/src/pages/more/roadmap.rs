crate::doc_page!(
    "Roadmap",
    "/ja/docs/more/roadmap",
    Content::new(vec![
        h2(vec![text("優先順位")]),
        p(vec![
            text("フレームワークの今後の機能やフォーカスの優先順位は、コミュニティによって決定されます。\
                 2020年の春には、プロジェクトの方向性についてのフィードバックを集めるために開発者アンケートが行われました。その概要は"),
            link(
                "https://github.com/yewstack/yew/wiki/Dev-Survey-%5BSpring-2020%5D",
                vec![text("Yew Wiki")]
            ),
            text("で見ることができます。"),
        ]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text("主要な取り組みの状況は、YewのGithubの"),
                link(
                    "https://github.com/yewstack/yew/projects",
                    vec![text("Project board")]
                ),
                text("で確認できます。"),
            ])]
        ),
        h2(vec![text("焦点")]),
        ol(vec![
            li(vec![text("Top Requested Features")]),
            li(vec![text("Production Readiness")]),
            li(vec![text("Documentation")]),
            li(vec![text("Pain Points")]),
        ]),
        h3(vec![text("Top Requested Features")]),
        ol(vec![
            li(vec![link(
                "https://github.com/yewstack/yew/projects/3",
                vec![text("関数型コンポーネント")]
            )]),
            li(vec![link(
                "https://github.com/yewstack/yew/projects/4",
                vec![text("Componentライブラリ")]
            )]),
            li(vec![text("より良い状態管理")]),
            li(vec![link(
                "https://github.com/yewstack/yew/projects/5",
                vec![text("サーバーサイドでのレンダリング")]
            )]),
        ]),
        h3(vec![text("Production Readiness")]),
        ul(vec![
            li(vec![text("テストカバレッジの向上")]),
            li(vec![text("バイナリサイズ")]),
            li(vec![link(
                "https://github.com/yewstack/yew/issues/5",
                vec![text("ベンチマークのパフォーマンス")]
            )]),
        ]),
        h3(vec![text("Documentation")]),
        ul(vec![
            li(vec![text("チュートリアルを作る")]),
            li(vec![text("プロジェクトのセットアップをシンプルにする")]),
        ]),
        h3(vec![text("Pain Points")]),
        ul(vec![
            li(vec![link(
                "https://github.com/yewstack/yew/issues/830",
                vec![text("Componentのボイラープレート")]
            )]),
            li(vec![text("Fetch API")]),
            li(vec![link(
                "https://github.com/yewstack/yew/projects/6",
                vec![text("エージェント")]
            )]),
        ]),
    ])
);
