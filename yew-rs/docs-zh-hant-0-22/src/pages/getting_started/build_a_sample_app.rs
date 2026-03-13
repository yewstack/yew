pub fn page_content() -> yew_site_lib::Content {
    yew_site_docs_zh_hant::pages::getting_started::build_a_sample_app::page_content_versioned(Some(
        "0.23",
    ))
}

crate::doc_page!(
    "建立一個範例應用",
    "/zh-Hant/docs/getting-started/build-a-sample-app",
    page_content()
);
