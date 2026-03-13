pub fn page_content() -> yew_site_lib::Content {
    yew_site_docs::pages::tutorial::page_content_versioned(Some("0.23"))
}

crate::doc_page!("Tutorial", "/docs/tutorial", page_content());
