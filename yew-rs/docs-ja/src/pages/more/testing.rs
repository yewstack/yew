pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition(
            AdmonitionType::Info,
            None,
            vec![
                p(vec![text(
                    "コンポーネントのテストをより簡単にするために努力していますが、\
                     現在進行中です。",
                )]),
                p(vec![
                    link(
                        "https://github.com/yewstack/yew/issues/1413",
                        vec![text("浅いレンダリング")],
                    ),
                    text(" のサポートは GitHub リポジトリで見つけることができます。"),
                ]),
            ],
        ),
        h2(vec![text("スナップショットテスト")]),
        p(vec![
            text("Yew はコンポーネントのスナップショットテストを容易にするために "),
            code("yew::tests::layout_tests"),
            text(" モジュールを提供しています。"),
        ]),
        admonition(
            AdmonitionType::Important,
            Some("ドキュメントの改善"),
            vec![p(vec![text(
                "スナップショットテストのドキュメントを改善するための助けが必要です。",
            )])],
        ),
        h2(vec![text("wasm_bindgen_test")]),
        p(vec![
            text("Rust/WASM ワーキンググループは "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                vec![code("wasm_bindgen_test")],
            ),
            text(" というクレートを維持しています。 これにより、組み込みの "),
            code("#[test]"),
            text(
                " プロシージャマクロに似た方法でブラウザ内でテストを実行できます。 \
                 このモジュールの詳細については、",
            ),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                vec![text("Rust Wasm ワーキンググループのドキュメント")],
            ),
            text(" を参照してください。"),
        ]),
    ])
}

crate::doc_page!(
    "アプリケーションのテスト",
    "/ja/docs/more/testing",
    page_content()
);
