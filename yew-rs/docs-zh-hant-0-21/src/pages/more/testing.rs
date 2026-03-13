crate::doc_page!(
    "測試",
    "/zh-Hant/docs/more/testing",
    Content::new(vec![
        h2(vec![text("wasm_bindgen_test")]),
        p(vec![
            text("Rust Wasm 工作群組有維護一個 crate 叫作 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                vec![code("wasm_bindgen_test")],
            ),
            text("，他讓你可以在瀏覽器裡跑類似於用內建的巨集 "),
            code("#[test]"),
            text(" 測試流程。更多資訊可以參考 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                vec![text("Rust Wasm working group's documentation")],
            ),
            text("。"),
        ]),
    ])
);
