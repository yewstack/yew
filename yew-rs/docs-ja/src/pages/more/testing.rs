pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition![
            AdmonitionType::Info,
            None,
            p![text(
                "コンポーネントのテストをより簡単にするために努力していますが、現在進行中です。",
            )],
            p![
                link!(
                    "https://github.com/yewstack/yew/issues/1413",
                    text("浅いレンダリング"),
                ),
                text(" のサポートは GitHub リポジトリで見つけることができます。"),
            ],
        ],
        h2![text("スナップショットテスト")],
        p![
            text("Yew はコンポーネントのスナップショットテストを容易にするために "),
            code("yew::tests::layout_tests"),
            text(" モジュールを提供しています。"),
        ],
        admonition![
            AdmonitionType::Important,
            Some("ドキュメントの改善"),
            p![text(
                "スナップショットテストのドキュメントを改善するための助けが必要です。",
            )],
        ],
        h2![text("wasm_bindgen_test")],
        p![
            text("Rust/WASM ワーキンググループは "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                code("wasm_bindgen_test"),
            ),
            text(" というクレートを維持しています。 これにより、組み込みの "),
            code("#[test]"),
            text(
                " プロシージャマクロに似た方法でブラウザ内でテストを実行できます。 \
                 このモジュールの詳細については、",
            ),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                text("Rust Wasm ワーキンググループのドキュメント"),
            ),
            text(" を参照してください。"),
        ],
    ])
}

crate::doc_page!(
    "アプリケーションのテスト",
    "/ja/docs/more/testing",
    page_content()
);
