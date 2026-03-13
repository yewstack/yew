crate::doc_page!(
    "Conditional Rendering",
    "/zh-Hans/docs/concepts/html/conditional-rendering",
    Content::new(vec![
        h2(vec![text("If blocks")]),
        p(vec![
            text("To conditionally render some markup, we wrap it in an "),
            code("if"),
            text(" block:")
        ]),
        p(vec![code("if"), text(":")]),
        p(vec![text("True case")]),
        p(vec![code("if - else"), text(":")]),
        p(vec![text("True case")]),
        p(vec![text("False case")]),
        p(vec![code("if let"), text(":")]),
        p(vec![text("{ text }")]),
        p(vec![code("if let else"), text(":")]),
        p(vec![text("{ text }")]),
        p(vec![text("False case")])
    ])
);
