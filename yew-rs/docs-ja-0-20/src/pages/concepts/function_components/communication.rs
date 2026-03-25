crate::doc_page!(
    "Communication between components",
    "/ja/docs/concepts/function-components/communication",
    Content::new(vec![
        h2![text("Parent to child messaging")],
        p![
            text("Pass data as "),
            link![
                "/ja/docs/concepts/function-components/properties",
                text("props")
            ],
            text(" that cause a rerender, this is the way to pass messages to children."),
        ],
        h2![text("Child to parent messaging")],
        p![
            text("Pass down a callback via props, that the child on an event can call. "),
            link![
                "/ja/docs/concepts/function-components/callbacks#passing-callbacks-as-props",
                text("Example")
            ],
        ],
    ])
);
