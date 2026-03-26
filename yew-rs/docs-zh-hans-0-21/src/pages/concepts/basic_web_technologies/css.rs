crate::doc_page!(
    "CSS with classes!",
    "/zh-Hans/docs/concepts/basic-web-technologies/css",
    Content::new(vec![
        p![code("class"), " attribute."],
        h2!["Classes"],
        p![
            "The ",
            code("classes!"),
            " macro and associated ",
            code("Classes"),
            " struct simplify the use of HTML classes:"
        ],
        h3!["Literal"],
        h3!["Multiple"],
        h3!["String"],
        h3!["Optional"],
        h3!["Vector"],
        h3!["Slice"],
        p![
            "We will expand upon this concept in ",
            link!("/docs/0.21/more/css", "more CSS"),
            "."
        ],
        h2!["Inline Styles"],
        p![
            "Currently Yew does not provide any special help with inline styles specified via the ",
            code("styles"),
            " attribute, but you can use it like any other HTML attribute:"
        ],
        p![
            "We will expand upon this concept in ",
            link!("/docs/0.21/more/css", "more CSS"),
            "."
        ]
    ])
);
