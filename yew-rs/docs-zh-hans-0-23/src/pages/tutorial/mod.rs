pub fn page_content() -> yew_site_lib::Content {
    yew_site_docs_zh_hans::pages::tutorial::page_content_versioned(Some("0.23"))
}

crate::doc_page!("教程", "/zh-Hans/docs/tutorial", page_content());
