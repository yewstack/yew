crate::doc_page!(
    "Roadmap",
    "/ja/docs/more/roadmap",
    Content::new(vec![
        h2![text("優先順位")],
        p![
            text("フレームワークの今後の機能やフォーカスの優先順位は、コミュニティによって決定されます。\
                 2020年の春には、プロジェクトの方向性についてのフィードバックを集めるために開発者アンケートが行われました。その概要は"),
            link!(
                "https://github.com/yewstack/yew/wiki/Dev-Survey-%5BSpring-2020%5D",
                text("Yew Wiki")
            ),
            text("で見ることができます。"),
        ],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                text("主要な取り組みの状況は、YewのGithubの"),
                link!(
                    "https://github.com/yewstack/yew/projects",
                    text("Project board")
                ),
                text("で確認できます。"),
            ],
        ],
        h2![text("焦点")],
        ol![
            li![text("Top Requested Features")],
            li![text("Production Readiness")],
            li![text("Documentation")],
            li![text("Pain Points")],
        ],
        h3![text("Top Requested Features")],
        ol![
            li![link!(
                "https://github.com/yewstack/yew/projects/3",
                text("関数型コンポーネント")
            )],
            li![link!(
                "https://github.com/yewstack/yew/projects/4",
                text("Componentライブラリ")
            )],
            li![text("より良い状態管理")],
            li![link!(
                "https://github.com/yewstack/yew/projects/5",
                text("サーバーサイドでのレンダリング")
            )],
        ],
        h3![text("Production Readiness")],
        ul![
            li![text("テストカバレッジの向上")],
            li![text("バイナリサイズ")],
            li![link!(
                "https://github.com/yewstack/yew/issues/5",
                text("ベンチマークのパフォーマンス")
            )],
        ],
        h3![text("Documentation")],
        ul![
            li![text("チュートリアルを作る")],
            li![text("プロジェクトのセットアップをシンプルにする")],
        ],
        h3![text("Pain Points")],
        ul![
            li![link!(
                "https://github.com/yewstack/yew/issues/830",
                text("Componentのボイラープレート")
            )],
            li![text("Fetch API")],
            li![link!(
                "https://github.com/yewstack/yew/projects/6",
                text("エージェント")
            )],
        ],
    ])
);
