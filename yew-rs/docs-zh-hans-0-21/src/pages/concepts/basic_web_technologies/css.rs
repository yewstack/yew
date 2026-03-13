crate::doc_page!(
    "CSS with classes!",
    "/zh-Hans/docs/concepts/basic-web-technologies/css",
    Content::new(vec![
        p(vec![code("class"), text(" attribute.")]),
        h2(vec![text("Classes")]),
        p(vec![
            text("The "),
            code("classes!"),
            text(" macro and associated "),
            code("Classes"),
            text(" struct simplify the use of HTML classes:")
        ]),
        h3(vec![text("Literal")]),
        h3(vec![text("Multiple")]),
        h3(vec![text("String")]),
        h3(vec![text("Optional")]),
        h3(vec![text("Vector")]),
        h3(vec![text("Slice")]),
        p(vec![
            text("We will expand upon this concept in "),
            link("/docs/0.21/more/css", vec![text("more CSS")]),
            text(".")
        ]),
        h2(vec![text("Inline Styles")]),
        p(vec![
            text(
                "Currently Yew does not provide any special help with inline styles specified via \
                 the "
            ),
            code("styles"),
            text(" attribute, but you can use it like any other HTML attribute:")
        ]),
        p(vec![
            text("We will expand upon this concept in "),
            link("/docs/0.21/more/css", vec![text("more CSS")]),
            text(".")
        ])
    ])
);
