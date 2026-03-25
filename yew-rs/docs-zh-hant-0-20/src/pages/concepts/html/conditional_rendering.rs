crate::doc_page!(
    "Conditional Rendering",
    "/zh-Hant/docs/concepts/html/conditional-rendering",
    Content::new(vec![
        h2![text("If blocks")],
        p![
            text("To conditionally render some markup, we wrap it in an "),
            code("if"),
            text(" block:"),
        ],
        tabs![
            "if",
            tab![
                "if",
                "if",
                code_block(
                    "rust",
                    "use yew::prelude::*;\n\nhtml! {\n    if true {\n        <p>{ \"True case\" \
                     }</p>\n    }\n};",
                ),
            ],
            tab![
                "if - else",
                "if - else",
                code_block(
                    "rust",
                    "use yew::prelude::*;\nlet some_condition = true;\n\nhtml! {\n    if \
                     some_condition {\n        <p>{ \"True case\" }</p>\n    } else {\n        \
                     <p>{ \"False case\" }</p>\n    }\n};",
                ),
            ],
            tab![
                "if let",
                "if let",
                code_block(
                    "rust",
                    "use yew::prelude::*;\nlet some_text = Some(\"text\");\n\nhtml! {\n    if let \
                     Some(text) = some_text {\n        <p>{ text }</p>\n    }\n};",
                ),
            ],
            tab![
                "if let else",
                "if let else",
                code_block(
                    "rust",
                    "use yew::prelude::*;\nlet some_text = Some(\"text\");\n\nhtml! {\n    if let \
                     Some(text) = some_text {\n        <p>{ text }</p>\n    } else {\n        \
                     <p>{ \"False case\" }</p>\n    }\n};",
                ),
            ],
        ],
    ])
);
