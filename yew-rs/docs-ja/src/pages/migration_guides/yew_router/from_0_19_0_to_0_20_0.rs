pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![p![
        text("破壊的変更はありません。"),
        code("Cargo.toml"),
        text(" で yew-router を 0.20.0 に更新してください。"),
    ]])
}

crate::doc_page!(
    "From 0.19.0 to 0.20.0",
    "/ja/docs/migration-guides/yew-router/from-0-19-0-to-0-20-0",
    page_content()
);
