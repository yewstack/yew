crate::doc_page!(
    "測試",
    "/zh-Hant/docs/more/testing",
    Content::new(vec![
        admonition(
            AdmonitionType::Info,
            None,
            vec![
                p(vec![text(
                    "我們正在努力使元件測試變得簡單，但目前仍在開發中。"
                )]),
                p(vec![
                    text("對"),
                    link(
                        "https://github.com/yewstack/yew/issues/1413",
                        vec![text("淺渲染")],
                    ),
                    text("的支持可以在 GitHub 儲存庫中找到。"),
                ]),
            ],
        ),
        h2(vec![text("快照測試")]),
        p(vec![
            text("Yew 提供了 "),
            code("yew::tests::layout_tests"),
            text(" 模組來方便元件的快照測試。"),
        ]),
        admonition(
            AdmonitionType::Warning,
            Some("貢獻"),
            vec![p(vec![text("幫助改進快照測試的文件。")])],
        ),
        h2(vec![text("wasm_bindgen_test")]),
        p(vec![
            text("Rust Wasm 工作群組有維護一個 crate 叫作 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                vec![text("wasm_bindgen_test")],
            ),
            text(" ，他讓你可以在瀏覽器裡跑類似於用內建的巨集"),
            code("#[test]"),
            text("測試流程。 更多資訊可以參考 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                vec![text("Rust Wasm working group's documentation")],
            ),
            text("。"),
        ]),
    ])
);
