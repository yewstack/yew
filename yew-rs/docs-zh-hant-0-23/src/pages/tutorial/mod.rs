pub fn page_content() -> yew_site_lib::Content {
    yew_site_docs_zh_hant::pages::tutorial::page_content_versioned(Some("0.23"))
}

crate::doc_page!("教學", "/zh-Hant/docs/tutorial", page_content());
