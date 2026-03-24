pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![
            text("Removal of "),
            code("Context"),
            text(" and "),
            code("Job"),
            text(" Agents"),
        ],
        p![
            text("The "),
            code("Context"),
            text(" and "),
            code("Job"),
            text(" Agents have been removed in favour of Yew's Context API."),
        ],
        p![
            text("You can see the updated "),
            link!(
                "https://github.com/yewstack/yew/tree/master/examples/pub_sub",
                code("pub_sub"),
            ),
            text(" which demonstrate how to use the context API."),
        ],
        p![
            text("For users of "),
            code("yew_agent::utils::store"),
            text(", you may switch to third party solutions like: "),
            link!("https://github.com/intendednull/yewdux", text("Yewdux"),),
            text(" or "),
            link!("https://github.com/futursolo/bounce", text("Bounce")),
            text("."),
        ],
        h2![
            code("Threaded"),
            text(" has been separated into "),
            code("PublicAgent"),
            text(" and "),
            code("PrivateAgent"),
        ],
        p![
            text("Replace "),
            code("use yew_agent::Threaded;"),
            text(" with "),
            code("use yew_agent::PublicAgent;"),
            text("."),
        ],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                code("Threaded"),
                text(
                    " was never implemented for Private Agents. All existing web worker-based \
                     agents are Public Agents.",
                ),
            ],
        ],
    ])
}

crate::doc_page!(
    "From 0.1.0 to 0.2.0",
    "/docs/migration-guides/yew-agent/from-0-1-0-to-0-2-0",
    page_content()
);
