crate::doc_page!(
    "Immutable Types",
    "/zh-Hans/docs/advanced-topics/immutable",
    Content::new(vec![
        h2!["What are immutable types?"],
        p![
            "These are types that you can instantiate but never mutate the values. In order to \
             update a value, you must instantiate a new value."
        ],
        h2!["Why using immutable types?"],
        p![
            "Properties, like in React, are propagated from ancestors to children. This means \
             that the properties must live when each component is updated. This is why properties \
             should ideally be cheap to clone. In order to achieve this we usually wrap things in ",
            code("Rc"),
            ".",
        ],
        p![
            "Immutable types are a great fit for holding property's values because they can be \
             cheaply cloned when passed from component to component."
        ],
        h2!["Further reading"],
        ul![
            li![link![
                "https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/immutable",
                "Immutable example",
            ]],
            li![link![
                "https://docs.rs/implicit-clone/",
                "Crate implicit-clone",
            ]],
        ],
    ])
    .with_description("Immutable data structures for Yew")
);
