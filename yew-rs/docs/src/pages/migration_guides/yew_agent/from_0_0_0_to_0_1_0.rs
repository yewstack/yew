pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            text("This is the first release of "),
            code("yew-agents"),
            text(" being separated from "),
            code("yew"),
        ],
        p![
            text("The only thing you will need to do is change the import paths from "),
            code("yew::*"),
            text(" to "),
            code("yew_agents::*"),
        ],
    ])
}

crate::doc_page!(
    "From 0.0.0 to 0.1.0",
    "/docs/migration-guides/yew-agent/from-0-0-0-to-0-1-0",
    page_content()
);
