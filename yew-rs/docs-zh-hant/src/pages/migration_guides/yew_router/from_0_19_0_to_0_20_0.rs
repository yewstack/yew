pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![p![
        text("沒有破壞性變更。在 "),
        code("Cargo.toml"),
        text(" 中將 yew-router 更新到 0.20.0。"),
    ]])
}

crate::doc_page!(
    "From 0.19.0 to 0.20.0",
    "/zh-Hant/docs/migration-guides/yew-router/from-0-19-0-to-0-20-0",
    page_content()
);
