pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["状態を保存するための一般的なビュー"],
        p!["この表は、どの状態保存タイプがあなたのユースケースに最適かを決定するためのガイドとして役立ちます："],
        table(
            vec![
                vec!["フック".into()],
                vec!["タイプ".into()],
                vec!["いつレンダリングされるか".into()],
                vec!["スコープ".into()],
            ],
            vec![
                vec![
                    vec![link!["https://yew-rs-api.web.app/next/yew/functional/fn.use_state.html", "use_state"]],
                    vec![code("T")],
                    vec!["値が設定されたとき".into()],
                    vec!["コンポーネントインスタンス内".into()],
                ],
                vec![
                    vec![link!["https://yew-rs-api.web.app/next/yew/functional/fn.use_state_eq.html", "use_state_eq"]],
                    vec![code("T: PartialEq")],
                    vec!["異なる値が設定されたとき".into()],
                    vec!["コンポーネントインスタンス内".into()],
                ],
                vec![
                    vec![link!["https://yew-rs-api.web.app/next/yew/functional/fn.use_reducer.html", "use_reducer"]],
                    vec![code("T: Reducible")],
                    vec!["リデューサーが呼び出されたとき".into()],
                    vec!["コンポーネントインスタンス内".into()],
                ],
                vec![
                    vec![link!["https://yew-rs-api.web.app/next/yew/functional/fn.use_reducer_eq.html", "use_reducer_eq"]],
                    vec![code("T: Reducible + PartialEq")],
                    vec!["リデューサーが呼び出され、結果が異なるとき".into()],
                    vec!["コンポーネントインスタンス内".into()],
                ],
                vec![
                    vec![link!["https://yew-rs-api.web.app/next/yew/functional/fn.use_memo.html", "use_memo"]],
                    vec![code("Deps -> T")],
                    vec!["依存関係が変わったとき".into()],
                    vec!["コンポーネントインスタンス内".into()],
                ],
                vec![
                    vec![link!["https://yew-rs-api.web.app/next/yew/functional/fn.use_callback.html", "use_callback"]],
                    vec![code("Deps -> Callback<E>")],
                    vec!["依存関係が変わったとき".into()],
                    vec!["コンポーネントインスタンス内".into()],
                ],
                vec![
                    vec![link!["https://yew-rs-api.web.app/next/yew/functional/fn.use_mut_ref.html", "use_mut_ref"]],
                    vec![code("T")],
                    vec!["-".into()],
                    vec!["コンポーネントインスタンス内".into()],
                ],
                vec![
                    vec!["グローバル静的定数".into()],
                    vec![code("T")],
                    vec!["-".into()],
                    vec!["グローバル、どこでも使用可能".into()],
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
