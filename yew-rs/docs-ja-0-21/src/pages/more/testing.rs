crate::doc_page!("Testing apps", "/ja/docs/more/testing", {
    Content::new(vec![
        p!["<TODO>"],
        h2!["wasm_bindgen_test"],
        p![
            "Rust Wasmワーキンググループは",
            code("wasm_bindgen_test"),
            "というフレームワークをメンテナンスしており、組み込みの",
            code("#[test]"),
            "プロシージャルマクロの動作と同様の方法でブラウザでテストを実行することができます。\
             詳細は、",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                "Rust Wasm活動グループのドキュメント"
            ),
            "に記載されています。",
        ],
    ])
    .with_description("アプリケーションをテストする")
});
