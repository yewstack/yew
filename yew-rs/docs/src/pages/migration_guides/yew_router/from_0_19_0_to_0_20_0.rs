pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![p![
        text("No breaking changes. Update yew-router to 0.20.0 in your "),
        code("Cargo.toml"),
        text("."),
    ]])
}

crate::doc_page!(
    "From 0.19.0 to 0.20.0",
    "/docs/migration-guides/yew-router/from-0-19-0-to-0-20-0",
    page_content()
);
