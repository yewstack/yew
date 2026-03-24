crate::doc_page!(
    "Conditional Rendering",
    "/zh-Hans/docs/concepts/html/conditional-rendering",
    Content::new(vec![
        h2![text("If blocks")],
        p![
            text("To conditionally render some markup, we wrap it in an "),
            code("if"),
            text(" block:")
        ],
        p![code("if"), text(":")],
        p![text("True case")],
        p![code("if - else"), text(":")],
        p![text("True case")],
        p![text("False case")],
        p![code("if let"), text(":")],
        p![text("{ text }")],
        p![code("if let else"), text(":")],
        p![text("{ text }")],
        p![text("False case")]
    ])
);
