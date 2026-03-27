pub fn page_content() -> yew_site_lib::Content {
    yew_site_lib::examples_page_content(true)
}

crate::doc_page!("Examples", "/docs/getting-started/examples", page_content());
