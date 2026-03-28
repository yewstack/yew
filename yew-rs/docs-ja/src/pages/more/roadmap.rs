pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["優先順位"],
        p![
            "フレームワークの今後の機能と重点の優先順位はコミュニティによって決定されます。 2020 年春に、プロジェクトの方向性に関するフィードバックを収集するために開発者調査を実施しました。 調査の概要は ",
            link!("https://github.com/yewstack/yew/wiki/Dev-Survey-%5BSpring-2020%5D", "Yew Wiki"),
            " で確認できます。",
        ],
        admonition![AdmonitionType::Note, None,
            p![
                "すべての主要なイニシアチブのステータスは Yew Github の ",
                link!("https://github.com/yewstack/yew/projects", "プロジェクトボード"),
                " で追跡できます。",
            ],
        ],
        h2!["重点"],
        ol![
            li!["最も人気のある機能"],
            li!["プロダクションレディ"],
            li!["ドキュメント"],
            li!["痛点"],
        ],
        h3!["最も人気のある機能"],
        ol![
            li![link!("https://github.com/yewstack/yew/projects/3", "関数コンポーネント")],
            li![link!("https://github.com/yewstack/yew/projects/4", "コンポーネントライブラリ")],
            li!["より良い状態管理"],
            li![link!("https://github.com/yewstack/yew/projects/5", "サーバーサイドレンダリング")],
        ],
        h3!["プロダクションレディに必要な問題"],
        ul![
            li!["Yew のテストカバレッジを向上させる"],
            li!["バイナリサイズを小さくする"],
            li![link!("https://github.com/yewstack/yew/issues/5", "パフォーマンスベンチマーク")],
        ],
        h3!["ドキュメント"],
        ul![
            li!["チュートリアルを作成する"],
            li!["プロジェクト設定を簡素化する"],
        ],
        h3!["痛点"],
        ul![
            li![link!("https://github.com/yewstack/yew/issues/830", "コンポーネントテンプレート")],
            li![link!("https://github.com/yewstack/yew/projects/6", "エージェント")],
        ],
    ])
    .with_description("Yew フレームワークの計画機能ロードマップ")
}

crate::doc_page!("ロードマップ", "/ja/docs/more/roadmap", page_content());
