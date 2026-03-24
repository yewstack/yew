pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("What are immutable types?")],
        p![text(
            "These are types that you can instantiate but never mutate the values. In order to \
             update a value, you must instantiate a new value.",
        )],
        h2![text("Why using immutable types?")],
        p![
            text(
                "Properties, like in React, are propagated from ancestors to children. This means \
                 that the properties must live when each component is updated. This is why \
                 properties should —ideally— be cheap to clone. To achieve this we usually wrap \
                 things in ",
            ),
            code("Rc"),
            text("."),
        ],
        p![text(
            "Immutable types are a great fit for holding property's values because they can be \
             cheaply cloned when passed from component to component.",
        )],
        h2![text("Common Immutable Types")],
        p![
            text("Yew recommends using the following immutable types from the "),
            code("implicit-clone"),
            text(" crate:"),
        ],
        ul![
            li![
                code("IString"),
                text(" (aliased as "),
                code("AttrValue"),
                text(" in Yew) - for strings instead of "),
                code("String"),
            ],
            li![
                code("IArray<T>"),
                text(" - for arrays/vectors instead of "),
                code("Vec<T>"),
            ],
            li![
                code("IMap<K, V>"),
                text(" - for maps instead of "),
                code("HashMap<K, V>"),
            ],
        ],
        p![
            text("These types are either reference-counted ("),
            code("Rc"),
            text(") or static references, making them very cheap to clone."),
        ],
        h2![text("Further reading")],
        ul![
            li![link![
                "https://github.com/yewstack/yew/tree/master/examples/immutable",
                text("Immutable example"),
            ]],
            li![link![
                "https://docs.rs/implicit-clone/",
                text("Crate "),
                code("implicit-clone"),
            ]],
        ],
    ])
}

crate::doc_page!(
    "Immutable Types",
    "/docs/advanced-topics/immutable",
    page_content()
);
