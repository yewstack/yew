pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![code("Switch::render"), " is no longer needed"],
        p![
            "The ",
            code("<Switch />"),
            " component now accepts a closure of ",
            code("Fn(Routable) -> Html"),
            " as the render function directly.",
        ],
        h2![code("navigator"), " API"],
        p!["The History API has been replaced with the Navigator API."],
    ])
}

crate::doc_page!(
    "From 0.16.0 to 0.17.0",
    "/docs/migration-guides/yew-router/from-0-16-0-to-0-17-0",
    page_content()
);
