pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![text("<TODO>")]),
        h2(vec![text("wasm_bindgen_test")]),
        p(vec![
            text("Rust Wasmワーキンググループは"),
            code("wasm_bindgen_test"),
            text("というフレームワークをメンテナンスしており、組み込みの"),
            code("#[test]"),
            text("プロシージャルマクロの動作と同様の方法でブラウザでテストを実行することができます。詳細は、"),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                vec![text("Rust Wasm活動グループのドキュメント")]
            ),
            text("に記載されています。"),
        ]),
    ])
}

crate::doc_page!("Testing apps", "/ja/docs/more/testing", page_content());
