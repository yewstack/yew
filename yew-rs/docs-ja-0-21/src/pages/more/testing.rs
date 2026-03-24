crate::doc_page!("Testing apps", "/ja/docs/more/testing", {
    Content::new(vec![
        p![text("<TODO>")],
        h2![text("wasm_bindgen_test")],
        p![
            text("Rust Wasmワーキンググループは"),
            code("wasm_bindgen_test"),
            text("というフレームワークをメンテナンスしており、組み込みの"),
            code("#[test]"),
            text("プロシージャルマクロの動作と同様の方法でブラウザでテストを実行することができます。詳細は、"),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                text("Rust Wasm活動グループのドキュメント")
            ),
            text("に記載されています。"),
        ],
    ])
});
