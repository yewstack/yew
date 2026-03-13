pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![code("Switch::render"), text(" is no longer needed")]),
        p(vec![
            text("The "),
            code("<Switch />"),
            text(" component now accepts a closure of "),
            code("Fn(Routable) -> Html"),
            text(" as the render function directly."),
        ]),
        h2(vec![code("navigator"), text(" API")]),
        p(vec![text(
            "The History API has been replaced with the Navigator API.",
        )]),
    ])
}

crate::doc_page!(
    "From 0.16.0 to 0.17.0",
    "/docs/migration-guides/yew-router/from-0-16-0-to-0-17-0",
    page_content()
);
