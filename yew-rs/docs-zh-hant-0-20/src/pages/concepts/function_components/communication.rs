crate::doc_page!(
    "Communication between components",
    "/zh-Hant/docs/concepts/function-components/communication",
    Content::new(vec![
        h2!["Parent to child messaging"],
        p![
            "Pass data as ",
            link!(
                "/zh-Hant/docs/concepts/function-components/properties",
                "props",
            ),
            " that cause a rerender, this is the way to pass messages to children.",
        ],
        h2!["Child to parent messaging"],
        p![
            "Pass down a callback via props, that the child on an event can call. ",
            link!(
                "/zh-Hant/docs/concepts/function-components/callbacks#passing-callbacks-as-props",
                "Example",
            ),
        ],
    ])
);
