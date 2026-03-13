crate::doc_page!(
    "Node Refs",
    "/ja/docs/concepts/function-components/node-refs",
    Content::new(vec![
        p(vec![
            text("The "),
            code("ref"),
            text(" attribute can be used to attach the "),
            code("NodeRef"),
            text(" to an HTML element. In callbacks, you can then get the DOM "),
            code("Element"),
            text(
                " that the ref is attached to. This can be used to make changes to the DOM \
                 outside of the "
            ),
            code("view"),
            text(" lifecycle method, retrieve the value of an "),
            code("<input>"),
            text(" and other direct interactions with the DOM via the javascript API."),
        ]),
        p(vec![text(
            "This is useful for getting ahold of canvas elements, or scrolling to different \
             sections of a page."
        )]),
        admonition(
            AdmonitionType::Caution,
            None,
            vec![p(vec![
                text("Do not manually modify the DOM tree that is rendered by Yew. Treat the "),
                code("NodeRef"),
                text(" as a read-only access, if you are unsure."),
            ]),]
        ),
        h2(vec![text("Further Reading")]),
        ul(vec![
            li(vec![link(
                "https://yew-rs-api.web.app/next/yew/functional/fn.use_node_ref.html",
                vec![text("use_node_ref hook")]
            ),]),
            li(vec![link(
                "https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/node_refs",
                vec![text("node_refs example")]
            ),]),
        ]),
    ])
);
