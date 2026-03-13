crate::doc_page!(
    "Testing apps",
    "/docs/more/testing",
    Content::new(vec![
        admonition(
            AdmonitionType::Info,
            None,
            vec![
                p(vec![text(
                    "We are working on making it easy to test components, but this is currently a \
                     work in progress."
                ),]),
                p(vec![
                    text("Support for "),
                    link(
                        "https://github.com/yewstack/yew/issues/1413",
                        vec![text("shallow rendering")]
                    ),
                    text(" can be found in the GitHub repository."),
                ]),
            ]
        ),
        h2(vec![text("Snapshot testing")]),
        p(vec![
            text("Yew exposes the "),
            code("yew::tests::layout_tests"),
            text(" module to facilitate snapshot testing of components."),
        ]),
        admonition(
            AdmonitionType::Warning,
            Some("Contribute"),
            vec![p(vec![text(
                "Help improve the documentation for snapshot testing."
            )]),]
        ),
        h2(vec![text("wasm_bindgen_test")]),
        p(vec![
            text("The Rust/WASM working group maintains a crate called "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                vec![code("wasm_bindgen_test"),]
            ),
            text(
                " which allows you to run tests in a browser in a similar fashion to how the \
                 built-in "
            ),
            code("#[test]"),
            text(" procedural macro works. More information is given in the "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                vec![text("Rust Wasm working group's documentation"),]
            ),
            text(" for this module."),
        ]),
    ])
);
