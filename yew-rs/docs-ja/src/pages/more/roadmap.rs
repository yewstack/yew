pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("優先順位")]),
        p(vec![
            text("フレームワークの今後の機能と重点の優先順位はコミュニティによって決定されます。 2020 年春に、プロジェクトの方向性に関するフィードバックを収集するために開発者調査を実施しました。 調査の概要は "),
            link("https://github.com/yewstack/yew/wiki/Dev-Survey-%5BSpring-2020%5D", vec![text("Yew Wiki")]),
            text(" で確認できます。"),
        ]),
        admonition(AdmonitionType::Note, None, vec![
            p(vec![
                text("すべての主要なイニシアチブのステータスは Yew Github の "),
                link("https://github.com/yewstack/yew/projects", vec![text("プロジェクトボード")]),
                text(" で追跡できます。"),
            ]),
        ]),
        h2(vec![text("重点")]),
        ol(vec![
            li(vec![text("最も人気のある機能")]),
            li(vec![text("プロダクションレディ")]),
            li(vec![text("ドキュメント")]),
            li(vec![text("痛点")]),
        ]),
        h3(vec![text("最も人気のある機能")]),
        ol(vec![
            li(vec![link("https://github.com/yewstack/yew/projects/3", vec![text("関数コンポーネント")])]),
            li(vec![link("https://github.com/yewstack/yew/projects/4", vec![text("コンポーネントライブラリ")])]),
            li(vec![text("より良い状態管理")]),
            li(vec![link("https://github.com/yewstack/yew/projects/5", vec![text("サーバーサイドレンダリング")])]),
        ]),
        h3(vec![text("プロダクションレディに必要な問題")]),
        ul(vec![
            li(vec![text("Yew のテストカバレッジを向上させる")]),
            li(vec![text("バイナリサイズを小さくする")]),
            li(vec![link("https://github.com/yewstack/yew/issues/5", vec![text("パフォーマンスベンチマーク")])]),
        ]),
        h3(vec![text("ドキュメント")]),
        ul(vec![
            li(vec![text("チュートリアルを作成する")]),
            li(vec![text("プロジェクト設定を簡素化する")]),
        ]),
        h3(vec![text("痛点")]),
        ul(vec![
            li(vec![link("https://github.com/yewstack/yew/issues/830", vec![text("コンポーネントテンプレート")])]),
            li(vec![link("https://github.com/yewstack/yew/projects/6", vec![text("エージェント")])]),
        ]),
    ])
}

crate::doc_page!("ロードマップ", "/ja/docs/more/roadmap", page_content());
