pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition![
            AdmonitionType::Info,
            None,
            p![text(
                "We are working on making it easy to test components, but this is currently a \
                 work in progress.",
            )],
            p![
                text("Support for "),
                link![
                    "https://github.com/yewstack/yew/issues/1413",
                    text("shallow rendering"),
                ],
                text(" can be found in the GitHub repository."),
            ],
        ],
        h2![text("Snapshot testing")],
        p![
            text("Yew exposes the "),
            code("yew::tests::layout_tests"),
            text(" module to facilitate snapshot testing of components."),
        ],
        admonition![
            AdmonitionType::Important,
            Some("contribute"),
            p![text("Help improve the documentation for snapshot testing.",)],
        ],
        h2![text("wasm_bindgen_test")],
        p![
            text("The Rust/WASM working group maintains a crate called "),
            link![
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                code("wasm_bindgen_test"),
            ],
            text(
                " which allows you to run tests in a browser in a similar fashion to how the \
                 built-in ",
            ),
            code("#[test]"),
            text(" procedural macro works. More information is given in the "),
            link![
                "https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/index.html",
                text("Rust Wasm working group's documentation"),
            ],
            text(" for this module."),
        ],
    ])
}

crate::doc_page!("Testing apps", "/docs/more/testing", page_content());
