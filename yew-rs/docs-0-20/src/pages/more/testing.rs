crate::doc_page!(
    "Testing apps",
    "/docs/more/testing",
    Content::new(vec![
        admonition![
            AdmonitionType::Info,
            None,
            p![
                "We are working on making it easy to test components, but this is currently a \
                 work in progress.",
            ],
            p![
                "Support for ",
                link!(
                    "https://github.com/yewstack/yew/issues/1413",
                    "shallow rendering",
                ),
                " can be found in the GitHub repository.",
            ],
        ],
        h2!["Snapshot testing"],
        p![
            "Yew exposes the ",
            code("yew::tests::layout_tests"),
            " module to facilitate snapshot testing of components.",
        ],
        admonition![
            AdmonitionType::Warning,
            Some("Contribute"),
            p!["Help improve the documentation for snapshot testing."],
        ],
        h2!["wasm_bindgen_test"],
        p![
            "The Rust/WASM working group maintains a crate called ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                code("wasm_bindgen_test"),
            ),
            " which allows you to run tests in a browser in a similar fashion to how the built-in ",
            code("#[test]"),
            " procedural macro works. More information is given in the ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                "Rust Wasm working group's documentation",
            ),
            " for this module.",
        ],
    ])
);
