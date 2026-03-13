pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![
            text("Removal of "),
            code("Context"),
            text(" and "),
            code("Job"),
            text(" Agents"),
        ]),
        p(vec![
            text("The "),
            code("Context"),
            text(" and "),
            code("Job"),
            text(" Agents have been removed in favour of Yew's Context API."),
        ]),
        p(vec![
            text("You can see the updated "),
            link(
                "https://github.com/yewstack/yew/tree/master/examples/pub_sub",
                vec![code("pub_sub")],
            ),
            text(" which demonstrate how to use the context API."),
        ]),
        p(vec![
            text("For users of "),
            code("yew_agent::utils::store"),
            text(", you may switch to third party solutions like: "),
            link(
                "https://github.com/intendednull/yewdux",
                vec![text("Yewdux")],
            ),
            text(" or "),
            link("https://github.com/futursolo/bounce", vec![text("Bounce")]),
            text("."),
        ]),
        h2(vec![
            code("Threaded"),
            text(" has been separated into "),
            code("PublicAgent"),
            text(" and "),
            code("PrivateAgent"),
        ]),
        p(vec![
            text("Replace "),
            code("use yew_agent::Threaded;"),
            text(" with "),
            code("use yew_agent::PublicAgent;"),
            text("."),
        ]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                code("Threaded"),
                text(
                    " was never implemented for Private Agents. All existing web worker-based \
                     agents are Public Agents.",
                ),
            ])],
        ),
    ])
}

crate::doc_page!(
    "From 0.1.0 to 0.2.0",
    "/docs/migration-guides/yew-agent/from-0-1-0-to-0-2-0",
    page_content()
);
