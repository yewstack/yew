pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("Parent to child messaging")],
        p![
            text("Pass data as "),
            link!(
                "/docs/concepts/function-components/properties",
                text("props"),
            ),
            text(" that cause a re-render, this is the way to pass messages to children."),
        ],
        h2![text("Child to parent messaging")],
        p![
            text("Pass down a callback via props, that the child on an event can call. "),
            link!(
                "/docs/concepts/function-components/callbacks#passing-callbacks-as-props",
                text("Example"),
            ),
        ],
    ])
}

crate::doc_page!(
    "Communication between components",
    "/docs/concepts/function-components/communication",
    page_content()
);
