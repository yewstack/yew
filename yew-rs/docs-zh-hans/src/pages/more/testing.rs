pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition!(
            AdmonitionType::Info,
            None,
            p!["我们正在努力让测试组件变得更容易，但目前仍在进行中。"],
            p![
                "在 GitHub 仓库中可以找到对 ",
                link!("https://github.com/yewstack/yew/issues/1413", "浅渲染",),
                " 的支持。",
            ],
        ),
        h2!["快照测试"],
        p![
            "Yew 提供了 ",
            code("yew::tests::layout_tests"),
            " 模块来方便组件的快照测试。",
        ],
        admonition!(
            AdmonitionType::Important,
            Some("改进文档"),
            p!["我们需要帮助，以改进快照测试的文档。"],
        ),
        h2!["wasm_bindgen_test"],
        p![
            "Rust/WASM 工作组维护了一个叫做 ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                code("wasm_bindgen_test"),
            ),
            " 的 crate， 它允许你以类似于内置的 ",
            code("#[test]"),
            " 过程宏的方式在浏览器中运行测试。 有关此模块的更多信息，请参阅 ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                "Rust Wasm 工作组的文档",
            ),
            "。",
        ],
    ])
    .with_description("测试你的应用")
}

crate::doc_page!("测试应用", "/zh-Hans/docs/more/testing", page_content());
