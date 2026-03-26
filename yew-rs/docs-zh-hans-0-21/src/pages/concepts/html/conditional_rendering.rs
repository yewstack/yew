crate::doc_page!(
    "Conditional Rendering",
    "/zh-Hans/docs/concepts/html/conditional-rendering",
    Content::new(vec![
        h2!["If blocks"],
        p![
            "To conditionally render some markup, we wrap it in an ",
            code("if"),
            " block:"
        ],
        p![code("if"), ":"],
        p!["True case"],
        p![code("if - else"), ":"],
        p!["True case"],
        p!["False case"],
        p![code("if let"), ":"],
        p!["{ text }"],
        p![code("if let else"), ":"],
        p!["{ text }"],
        p!["False case"]
    ])
);
