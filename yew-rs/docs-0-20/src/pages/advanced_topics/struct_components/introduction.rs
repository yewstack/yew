crate::doc_page!(
    "Introduction",
    "/docs/advanced-topics/struct-components",
    Content::new(vec![
        h2(vec![text("What are Components?")]),
        p(vec![
            text(
                "Components are the building blocks of Yew. They manage their own state and can \
                 render themselves to the DOM. Components are created by implementing the "
            ),
            code("Component"),
            text(" trait for a type."),
        ]),
        h2(vec![text("Writing Component's markup")]),
        p(vec![
            text(
                "Yew uses Virtual DOM to render elements to the DOM. The Virtual DOM tree can be \
                 constructed by using the "
            ),
            code("html!"),
            text(" macro. "),
            code("html!"),
            text(
                " uses syntax which is similar to HTML but is not exactly the same. The rules are \
                 also much stricter. It also provides super-powers like conditional rendering and \
                 rendering of lists using iterators."
            ),
        ]),
        admonition(
            AdmonitionType::Info,
            None,
            vec![p(vec![link(
                "/docs/concepts/html",
                vec![text(
                    "Learn more about the html! macro, how it's used and its syntax"
                )]
            ),]),]
        ),
        h2(vec![text("Passing data to a component")]),
        p(vec![
            text("Yew components use "),
            italic(vec![text("props")]),
            text(
                " to communicate between parent and children. A parent component may pass any \
                 data as props to its children. Props are similar to HTML attributes but any Rust \
                 type can be passed as props."
            ),
        ]),
        admonition(
            AdmonitionType::Info,
            None,
            vec![p(vec![link(
                "/docs/advanced-topics/struct-components/properties",
                vec![text("Learn more about the props")]
            ),]),]
        ),
        admonition(
            AdmonitionType::Info,
            None,
            vec![p(vec![
                text("For other than parent/child communication, use "),
                link("/docs/concepts/contexts", vec![text("contexts")]),
            ]),]
        ),
    ])
);
