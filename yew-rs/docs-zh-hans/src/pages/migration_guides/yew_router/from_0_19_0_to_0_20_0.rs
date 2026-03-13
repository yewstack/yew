pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![p(vec![
        text("没有破坏性变更。在 "),
        code("Cargo.toml"),
        text(" 中将 yew-router 更新到 0.20.0。"),
    ])])
}

crate::doc_page!(
    "From 0.19.0 to 0.20.0",
    "/zh-Hans/docs/migration-guides/yew-router/from-0-19-0-to-0-20-0",
    page_content()
);
