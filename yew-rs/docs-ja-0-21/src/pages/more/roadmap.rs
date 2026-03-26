crate::doc_page!(
    "Roadmap",
    "/ja/docs/more/roadmap",
    Content::new(vec![
        h2!["優先順位"],
        p![
            "フレームワークの今後の機能やフォーカスの優先順位は、コミュニティによって決定されます。\
                 2020年の春には、プロジェクトの方向性についてのフィードバックを集めるために開発者アンケートが行われました。その概要は",
            link!(
                "https://github.com/yewstack/yew/wiki/Dev-Survey-%5BSpring-2020%5D",
                "Yew Wiki"
            ),
            "で見ることができます。",
        ],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                "主要な取り組みの状況は、YewのGithubの",
                link!(
                    "https://github.com/yewstack/yew/projects",
                    "Project board"
                ),
                "で確認できます。",
            ],
        ],
        h2!["焦点"],
        ol![
            li!["Top Requested Features"],
            li!["Production Readiness"],
            li!["Documentation"],
            li!["Pain Points"],
        ],
        h3!["Top Requested Features"],
        ol![
            li![link!(
                "https://github.com/yewstack/yew/projects/3",
                "関数型コンポーネント"
            )],
            li![link!(
                "https://github.com/yewstack/yew/projects/4",
                "Componentライブラリ"
            )],
            li!["より良い状態管理"],
            li![link!(
                "https://github.com/yewstack/yew/projects/5",
                "サーバーサイドでのレンダリング"
            )],
        ],
        h3!["Production Readiness"],
        ul![
            li!["テストカバレッジの向上"],
            li!["バイナリサイズ"],
            li![link!(
                "https://github.com/yewstack/yew/issues/5",
                "ベンチマークのパフォーマンス"
            )],
        ],
        h3!["Documentation"],
        ul![
            li!["チュートリアルを作る"],
            li!["プロジェクトのセットアップをシンプルにする"],
        ],
        h3!["Pain Points"],
        ul![
            li![link!(
                "https://github.com/yewstack/yew/issues/830",
                "Componentのボイラープレート"
            )],
            li!["Fetch API"],
            li![link!(
                "https://github.com/yewstack/yew/projects/6",
                "エージェント"
            )],
        ],
    ])
);
