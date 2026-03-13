crate::doc_page!(
    "Communication between components",
    "/docs/concepts/function-components/communication",
    Content::new(vec![
        h2(vec![text("Parent to child messaging")]),
        p(vec![
            text("Pass data as "),
            link(
                "/docs/concepts/function-components/properties",
                vec![text("props")]
            ),
            text(" that cause a rerender, this is the way to pass messages to children."),
        ]),
        h2(vec![text("Child to parent messaging")]),
        p(vec![
            text("Pass down a callback via props, that the child on an event can call. "),
            link(
                "/docs/concepts/function-components/callbacks#passing-callbacks-as-props",
                vec![text("Example")]
            ),
        ]),
    ])
);
