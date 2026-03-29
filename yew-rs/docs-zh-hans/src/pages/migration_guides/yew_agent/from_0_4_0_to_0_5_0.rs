pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![p![
        "没有破坏性变更。在 ",
        code("Cargo.toml"),
        " 中将 yew-agent 更新到 0.5.0。",
    ]])
}

crate::doc_page!(
    "From 0.4.0 to 0.5.0",
    "/zh-Hans/docs/migration-guides/yew-agent/from-0-4-0-to-0-5-0",
    page_content()
);
