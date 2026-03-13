pub fn page_content() -> yew_site_lib::Content {
    yew_site_docs_ja::pages::getting_started::build_a_sample_app::page_content_versioned(Some(
        "0.23",
    ))
}

crate::doc_page!(
    "サンプルアプリケーションの構築",
    "/ja/docs/getting-started/build-a-sample-app",
    page_content()
);
