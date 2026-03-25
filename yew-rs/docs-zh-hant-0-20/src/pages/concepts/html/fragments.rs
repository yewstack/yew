crate::doc_page!(
    "Fragments",
    "/zh-Hant/docs/concepts/html/fragments",
    Content::new(vec![
        p![
            text("The "),
            code("html!"),
            text(
                " macro always requires a single root node. In order to get around this \
                 restriction, you can use an \"empty tag\" (these are also called \"fragments\")."
            ),
        ],
        tabs![
            "Valid",
            tab![
                "Valid",
                "Valid",
                code_block(
                    "rust",
                    "use yew::prelude::*;\n\nhtml! {\n    <>\n        <div></div>\n        \
                     <p></p>\n    </>\n};",
                ),
            ],
            tab![
                "Invalid",
                "Invalid",
                code_block_compile_fail(
                    "rust",
                    "use yew::prelude::*;\n\n// error: only one root html element \
                     allowed\n\nhtml! {\n    <div></div>\n    <p></p>\n};",
                ),
            ],
        ],
    ])
);
