pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition!(
            AdmonitionType::Info,
            None,
            p!["我們正在努力讓測試組件變得更容易，但目前仍在進行中。"],
            p![
                "在 GitHub 倉庫中可以找到對 ",
                link!("https://github.com/yewstack/yew/issues/1413", "淺渲染",),
                " 的支援。",
            ],
        ),
        h2!["快照測試"],
        p![
            "Yew 提供了 ",
            code("yew::tests::layout_tests"),
            " 模組來方便元件的快照測試。",
        ],
        admonition!(
            AdmonitionType::Important,
            Some("改進文檔"),
            p!["我們需要幫助，以改進快照測試的文件。"],
        ),
        h2!["wasm_bindgen_test"],
        p![
            "Rust/WASM 工作小組維護了一個稱為 ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                code("wasm_bindgen_test"),
            ),
            " 的 crate， 它允許你以類似於內建的 ",
            code("#[test]"),
            " 過程巨集的方式在瀏覽器中執行測試。 有關此模組的更多信息，請參閱 ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                "Rust Wasm 工作組的文檔",
            ),
            "。",
        ],
    ])
}

crate::doc_page!("測試應用", "/zh-Hant/docs/more/testing", page_content());
