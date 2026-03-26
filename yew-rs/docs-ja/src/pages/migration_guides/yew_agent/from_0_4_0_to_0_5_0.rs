pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![p![
        "破壊的変更はありません。",
        code("Cargo.toml"),
        " で yew-agent を 0.5.0 に更新してください。",
    ]])
}

crate::doc_page!(
    "From 0.4.0 to 0.5.0",
    "/ja/docs/migration-guides/yew-agent/from-0-4-0-to-0-5-0",
    page_content()
);
