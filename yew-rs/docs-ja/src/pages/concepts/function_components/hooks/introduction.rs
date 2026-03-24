pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("Hooks")],
        p![text("Hooks は、状態を保存し、副作用を実行することができる関数の一種です。")],
        p![
            text("Yew はいくつかの事前定義された hooks を提供しています。また、自分で hooks を作成することもできますし、多くの"),
            link!["/community/awesome#hooks", text("コミュニティ製の hooks")],
            text(" を見つけることもできます。"),
        ],
        h2![text("Hooks のルール")],
        ol![
            li![
                text("各 Hook 関数の名前は "),
                code("use_"),
                text(" で始める必要があります"),
            ],
            li_blocks![
                p![text("Hooks は次の場所でのみ使用できます：")],
                ul![
                    li![text("関数/ Hook のトップレベル")],
                    li![text("関数/ Hook 内のブロック、ただし分岐していない場合")],
                    li![
                        text("関数/ Hook 内トップレベルの "),
                        code("if"),
                        text(" 式の条件"),
                    ],
                    li![
                        text("関数/ Hook 内トップレベルの "),
                        code("match"),
                        text(" 式のセレクター"),
                    ],
                ],
            ],
            li![
                text("各レンダリング時に、Hooks は同じ順序で呼び出される必要があります。"),
                link!["/ja/docs/concepts/suspense", text("Suspense")],
                text(" を使用する場合のみ、早期リターンが許可されます"),
            ],
        ],
        p![text("これらのルールは、コンパイル時または実行時のエラーによって強制されます。")],
        h3![text("事前定義された Hooks")],
        p![text("Yew は次の事前定義された Hooks を提供しています：")],
        ul![
            li![code("use_state")],
            li![code("use_state_eq")],
            li![code("use_memo")],
            li![code("use_callback")],
            li![code("use_ref")],
            li![code("use_mut_ref")],
            li![code("use_node_ref")],
            li![code("use_reducer")],
            li![code("use_reducer_eq")],
            li![code("use_effect")],
            li![code("use_effect_with")],
            li![code("use_context")],
            li![code("use_force_update")],
        ],
        p![
            text("これらの hooks のドキュメントは "),
            link!["https://yew-rs-api.web.app/next/yew/functional/", text("Yew API ドキュメント")],
            text("で見つけることができます。"),
        ],
        h3![text("カスタム Hooks")],
        p![text("場合によっては、独自の Hooks を定義して、コンポーネント内の状態を持つ可能性のあるロジックを再利用可能な関数にカプセル化することが望ましいことがあります。")],
        h2![text("さらなる読み物")],
        ul![
            li![
                text("React ドキュメントには "),
                link!["https://reactjs.org/docs/hooks-intro.html", text("React hooks")],
                text(" に関するセクションがあります。"),
            ],
        ],
    ])
}

crate::doc_page!(
    "Hooks",
    "/ja/docs/concepts/function-components/hooks",
    page_content()
);
