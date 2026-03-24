pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("状態を保存するための一般的なビュー")],
        p![text("この表は、どの状態保存タイプがあなたのユースケースに最適かを決定するためのガイドとして役立ちます：")],
        table(
            vec![
                vec![text("フック")],
                vec![text("タイプ")],
                vec![text("いつレンダリングされるか")],
                vec![text("スコープ")],
            ],
            vec![
                vec![
                    vec![link!["https://yew-rs-api.web.app/next/yew/functional/fn.use_state.html", text("use_state")]],
                    vec![code("T")],
                    vec![text("値が設定されたとき")],
                    vec![text("コンポーネントインスタンス内")],
                ],
                vec![
                    vec![link!["https://yew-rs-api.web.app/next/yew/functional/fn.use_state_eq.html", text("use_state_eq")]],
                    vec![code("T: PartialEq")],
                    vec![text("異なる値が設定されたとき")],
                    vec![text("コンポーネントインスタンス内")],
                ],
                vec![
                    vec![link!["https://yew-rs-api.web.app/next/yew/functional/fn.use_reducer.html", text("use_reducer")]],
                    vec![code("T: Reducible")],
                    vec![text("リデューサーが呼び出されたとき")],
                    vec![text("コンポーネントインスタンス内")],
                ],
                vec![
                    vec![link!["https://yew-rs-api.web.app/next/yew/functional/fn.use_reducer_eq.html", text("use_reducer_eq")]],
                    vec![code("T: Reducible + PartialEq")],
                    vec![text("リデューサーが呼び出され、結果が異なるとき")],
                    vec![text("コンポーネントインスタンス内")],
                ],
                vec![
                    vec![link!["https://yew-rs-api.web.app/next/yew/functional/fn.use_memo.html", text("use_memo")]],
                    vec![code("Deps -> T")],
                    vec![text("依存関係が変わったとき")],
                    vec![text("コンポーネントインスタンス内")],
                ],
                vec![
                    vec![link!["https://yew-rs-api.web.app/next/yew/functional/fn.use_callback.html", text("use_callback")]],
                    vec![code("Deps -> Callback<E>")],
                    vec![text("依存関係が変わったとき")],
                    vec![text("コンポーネントインスタンス内")],
                ],
                vec![
                    vec![link!["https://yew-rs-api.web.app/next/yew/functional/fn.use_mut_ref.html", text("use_mut_ref")]],
                    vec![code("T")],
                    vec![text("-")],
                    vec![text("コンポーネントインスタンス内")],
                ],
                vec![
                    vec![text("グローバル静的定数")],
                    vec![code("T")],
                    vec![text("-")],
                    vec![text("グローバル、どこでも使用可能")],
                ],
            ],
        ),
    ])
}

crate::doc_page!(
    "状態",
    "/ja/docs/concepts/function-components/state",
    page_content()
);
