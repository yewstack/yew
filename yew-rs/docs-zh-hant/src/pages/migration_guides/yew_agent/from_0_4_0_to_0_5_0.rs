pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![p![
        "沒有破壞性變更。在 ",
        code("Cargo.toml"),
        " 中將 yew-agent 更新到 0.5.0。",
    ]])
}

crate::doc_page!(
    "From 0.4.0 to 0.5.0",
    "/zh-Hant/docs/migration-guides/yew-agent/from-0-4-0-to-0-5-0",
    page_content()
);
