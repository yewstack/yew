pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![p(vec![
        text("No breaking changes. Update yew-agent to 0.5.0 in your "),
        code("Cargo.toml"),
        text("."),
    ])])
}

crate::doc_page!(
    "From 0.4.0 to 0.5.0",
    "/docs/migration-guides/yew-agent/from-0-4-0-to-0-5-0",
    page_content()
);
