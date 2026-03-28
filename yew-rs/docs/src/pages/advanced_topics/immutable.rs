pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
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
             should —ideally— be cheap to clone. To achieve this we usually wrap things in ",
            code("Rc"),
            ".",
        ],
        p![
            "Immutable types are a great fit for holding property's values because they can be \
             cheaply cloned when passed from component to component."
        ],
        h2!["Common Immutable Types"],
        p![
            "Yew recommends using the following immutable types from the ",
            code("implicit-clone"),
            " crate:",
        ],
        ul![
            li![
                code("IString"),
                " (aliased as ",
                code("AttrValue"),
                " in Yew) - for strings instead of ",
                code("String"),
            ],
            li![
                code("IArray<T>"),
                " - for arrays/vectors instead of ",
                code("Vec<T>"),
            ],
            li![
                code("IMap<K, V>"),
                " - for maps instead of ",
                code("HashMap<K, V>"),
            ],
        ],
        p![
            "These types are either reference-counted (",
            code("Rc"),
            ") or static references, making them very cheap to clone.",
        ],
        h2!["Further reading"],
        ul![
            li![link![
                "https://github.com/yewstack/yew/tree/master/examples/immutable",
                "Immutable example",
            ]],
            li![link![
                "https://docs.rs/implicit-clone/",
                "Crate ",
                code("implicit-clone"),
            ]],
        ],
    ])
    .with_description("Immutable data structures for Yew")
}

crate::doc_page!(
    "Immutable Types",
    "/docs/advanced-topics/immutable",
    page_content()
);
