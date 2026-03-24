pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("優先順位")],
        p![
            text("フレームワークの今後の機能と重点の優先順位はコミュニティによって決定されます。 2020 年春に、プロジェクトの方向性に関するフィードバックを収集するために開発者調査を実施しました。 調査の概要は "),
            link!("https://github.com/yewstack/yew/wiki/Dev-Survey-%5BSpring-2020%5D", text("Yew Wiki")),
            text(" で確認できます。"),
        ],
        admonition![AdmonitionType::Note, None,
            p![
                text("すべての主要なイニシアチブのステータスは Yew Github の "),
                link!("https://github.com/yewstack/yew/projects", text("プロジェクトボード")),
                text(" で追跡できます。"),
            ],
        ],
        h2![text("重点")],
        ol![
            li![text("最も人気のある機能")],
            li![text("プロダクションレディ")],
            li![text("ドキュメント")],
            li![text("痛点")],
        ],
        h3![text("最も人気のある機能")],
        ol![
            li![link!("https://github.com/yewstack/yew/projects/3", text("関数コンポーネント"))],
            li![link!("https://github.com/yewstack/yew/projects/4", text("コンポーネントライブラリ"))],
            li![text("より良い状態管理")],
            li![link!("https://github.com/yewstack/yew/projects/5", text("サーバーサイドレンダリング"))],
        ],
        h3![text("プロダクションレディに必要な問題")],
        ul![
            li![text("Yew のテストカバレッジを向上させる")],
            li![text("バイナリサイズを小さくする")],
            li![link!("https://github.com/yewstack/yew/issues/5", text("パフォーマンスベンチマーク"))],
        ],
        h3![text("ドキュメント")],
        ul![
            li![text("チュートリアルを作成する")],
            li![text("プロジェクト設定を簡素化する")],
        ],
        h3![text("痛点")],
        ul![
            li![link!("https://github.com/yewstack/yew/issues/830", text("コンポーネントテンプレート"))],
            li![link!("https://github.com/yewstack/yew/projects/6", text("エージェント"))],
        ],
    ])
}

crate::doc_page!("ロードマップ", "/ja/docs/more/roadmap", page_content());
