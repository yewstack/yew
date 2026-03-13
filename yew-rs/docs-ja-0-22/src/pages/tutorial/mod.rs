pub fn page_content() -> yew_site_lib::Content {
    yew_site_docs_ja::pages::tutorial::page_content_versioned(Some("0.22"))
}

crate::doc_page!("チュートリアル", "/ja/docs/tutorial", page_content());
