pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("What are immutable types?")]),
        p(vec![text(
            "These are types that you can instantiate but never mutate the values. In order to \
             update a value, you must instantiate a new value.",
        )]),
        h2(vec![text("Why using immutable types?")]),
        p(vec![
            text(
                "Properties, like in React, are propagated from ancestors to children. This means \
                 that the properties must live when each component is updated. This is why \
                 properties should \u{2014}ideally\u{2014} be cheap to clone. To achieve this we \
                 usually wrap things in ",
            ),
            code("Rc"),
            text("."),
        ]),
        p(vec![text(
            "Immutable types are a great fit for holding property's values because they can be \
             cheaply cloned when passed from component to component.",
        )]),
        h2(vec![text("Common Immutable Types")]),
        p(vec![
            text("Yew recommends using the following immutable types from the "),
            code("implicit-clone"),
            text(" crate:"),
        ]),
        ul(vec![
            li(vec![
                code("IString"),
                text(" (aliased as "),
                code("AttrValue"),
                text(" in Yew) - for strings instead of "),
                code("String"),
            ]),
            li(vec![
                code("IArray<T>"),
                text(" - for arrays/vectors instead of "),
                code("Vec<T>"),
            ]),
            li(vec![
                code("IMap<K, V>"),
                text(" - for maps instead of "),
                code("HashMap<K, V>"),
            ]),
        ]),
        p(vec![
            text("These types are either reference-counted ("),
            code("Rc"),
            text(") or static references, making them very cheap to clone."),
        ]),
        h2(vec![text("Further reading")]),
        ul(vec![
            li(vec![link(
                "https://github.com/yewstack/yew/tree/master/examples/immutable",
                vec![text("Immutable example")],
            )]),
            li(vec![link(
                "https://docs.rs/implicit-clone/",
                vec![text("Crate "), code("implicit-clone")],
            )]),
        ]),
    ])
}

crate::doc_page!(
    "Immutable Types",
    "/docs/advanced-topics/immutable",
    page_content()
);
