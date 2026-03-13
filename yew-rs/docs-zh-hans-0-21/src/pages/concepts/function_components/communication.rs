crate::doc_page!(
    "Communication between components",
    "/zh-Hans/docs/concepts/function-components/communication",
    Content::new(vec![
        h2(vec![text("Parent to child messaging")]),
        p(vec![
            text("Pass data as "),
            link(
                "/docs/0.21/concepts/function-components/properties",
                vec![text("props")]
            ),
            text(" that cause a re-render, this is the way to pass messages to children.")
        ]),
        h2(vec![text("Child to parent messaging")]),
        p(vec![
            text("Pass down a callback via props, that the child on an event can call. "),
            link(
                "/docs/0.21/concepts/function-components/callbacks#passing-callbacks-as-props",
                vec![text("Example")]
            )
        ])
    ])
);
