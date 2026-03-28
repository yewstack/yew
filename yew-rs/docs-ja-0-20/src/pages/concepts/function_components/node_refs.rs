crate::doc_page!(
    "Node Refs",
    "/ja/docs/concepts/function-components/node-refs",
    Content::new(vec![
        p![
            "The ",
            code("ref"),
            " attribute can be used to attach the ",
            code("NodeRef"),
            " to an HTML element. In callbacks, you can then get the DOM ",
            code("Element"),
            " that the ref is attached to. This can be used to make changes to the DOM outside of \
             the ",
            code("view"),
            " lifecycle method, retrieve the value of an ",
            code("<input>"),
            " and other direct interactions with the DOM via the javascript API.",
        ],
        p![
            "This is useful for getting ahold of canvas elements, or scrolling to different \
             sections of a page."
        ],
        admonition![
            AdmonitionType::Caution,
            None,
            p![
                "Do not manually modify the DOM tree that is rendered by Yew. Treat the ",
                code("NodeRef"),
                " as a read-only access, if you are unsure.",
            ]
        ],
        h2!["Further Reading"],
        ul![
            li![link![
                "https://yew-rs-api.web.app/next/yew/functional/fn.use_node_ref.html",
                "use_node_ref hook"
            ]],
            li![link![
                "https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/node_refs",
                "node_refs example"
            ]],
        ],
    ])
    .with_description("Out-of-band DOM access")
);
