pub fn page_content() -> yew_site_lib::Content {
    yew_site_docs_zh_hans::pages::getting_started::build_a_sample_app::page_content_versioned(Some(
        "0.23",
    ))
}

crate::doc_page!(
    "构建一个示例应用",
    "/zh-Hans/docs/getting-started/build-a-sample-app",
    page_content()
);
