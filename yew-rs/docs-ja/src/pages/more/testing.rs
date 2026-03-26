pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition![
            AdmonitionType::Info,
            None,
            p!["コンポーネントのテストをより簡単にするために努力していますが、現在進行中です。"],
            p![
                link!(
                    "https://github.com/yewstack/yew/issues/1413",
                    "浅いレンダリング",
                ),
                " のサポートは GitHub リポジトリで見つけることができます。",
            ],
        ],
        h2!["スナップショットテスト"],
        p![
            "Yew はコンポーネントのスナップショットテストを容易にするために ",
            code("yew::tests::layout_tests"),
            " モジュールを提供しています。",
        ],
        admonition![
            AdmonitionType::Important,
            Some("ドキュメントの改善"),
            p!["スナップショットテストのドキュメントを改善するための助けが必要です。"],
        ],
        h2!["wasm_bindgen_test"],
        p![
            "Rust/WASM ワーキンググループは ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                code("wasm_bindgen_test"),
            ),
            " というクレートを維持しています。 これにより、組み込みの ",
            code("#[test]"),
            " プロシージャマクロに似た方法でブラウザ内でテストを実行できます。 \
             このモジュールの詳細については、",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                "Rust Wasm ワーキンググループのドキュメント",
            ),
            " を参照してください。",
        ],
    ])
}

crate::doc_page!(
    "アプリケーションのテスト",
    "/ja/docs/more/testing",
    page_content()
);
