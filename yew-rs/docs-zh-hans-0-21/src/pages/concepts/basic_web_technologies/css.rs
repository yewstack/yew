crate::doc_page!(
    "CSS with classes!",
    "/zh-Hans/docs/concepts/basic-web-technologies/css",
    Content::new(vec![
        p![code("class"), text(" attribute.")],
        h2![text("Classes")],
        p![
            text("The "),
            code("classes!"),
            text(" macro and associated "),
            code("Classes"),
            text(" struct simplify the use of HTML classes:")
        ],
        h3![text("Literal")],
        h3![text("Multiple")],
        h3![text("String")],
        h3![text("Optional")],
        h3![text("Vector")],
        h3![text("Slice")],
        p![
            text("We will expand upon this concept in "),
            link!("/docs/0.21/more/css", text("more CSS")),
            text(".")
        ],
        h2![text("Inline Styles")],
        p![
            text(
                "Currently Yew does not provide any special help with inline styles specified via \
                 the "
            ),
            code("styles"),
            text(" attribute, but you can use it like any other HTML attribute:")
        ],
        p![
            text("We will expand upon this concept in "),
            link!("/docs/0.21/more/css", text("more CSS")),
            text(".")
        ]
    ])
);
