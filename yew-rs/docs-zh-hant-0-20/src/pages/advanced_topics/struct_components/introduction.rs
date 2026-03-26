crate::doc_page!(
    "Introduction",
    "/zh-Hant/docs/advanced-topics/struct-components",
    Content::new(vec![
        h2!["What are Components?"],
        p![
            "Components are the building blocks of Yew. They manage their own state and can \
             render themselves to the DOM. Components are created by implementing the ",
            code("Component"),
            " trait for a type.",
        ],
        h2!["Writing Component's markup"],
        p![
            "Yew uses Virtual DOM to render elements to the DOM. The Virtual DOM tree can be \
             constructed by using the ",
            code("html!"),
            " macro. ",
            code("html!"),
            " uses syntax which is similar to HTML but is not exactly the same. The rules are \
             also much stricter. It also provides super-powers like conditional rendering and \
             rendering of lists using iterators.",
        ],
        admonition![
            AdmonitionType::Info,
            None,
            p![link![
                "/zh-Hant/docs/concepts/html",
                "Learn more about the html! macro, how it's used and its syntax"
            ],],
        ],
        h2!["Passing data to a component"],
        p![
            "Yew components use ",
            italic!["props"],
            " to communicate between parent and children. A parent component may pass any data as \
             props to its children. Props are similar to HTML attributes but any Rust type can be \
             passed as props.",
        ],
        admonition![
            AdmonitionType::Info,
            None,
            p![link![
                "/zh-Hant/docs/advanced-topics/struct-components/properties",
                "Learn more about the props"
            ],],
        ],
        admonition![
            AdmonitionType::Info,
            None,
            p![
                "For other than parent/child communication, use ",
                doc_link![crate::pages::concepts::contexts, "contexts"],
            ],
        ],
    ])
);
