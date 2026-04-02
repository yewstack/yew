pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![
            "Removal of ",
            code("Context"),
            " and ",
            code("Job"),
            " Agents",
        ],
        p![
            "The ",
            code("Context"),
            " and ",
            code("Job"),
            " Agents have been removed in favour of Yew's Context API.",
        ],
        p![
            "You can see the updated ",
            link!(
                "https://github.com/yewstack/yew/tree/master/examples/pub_sub",
                code("pub_sub"),
            ),
            " which demonstrate how to use the context API.",
        ],
        p![
            "For users of ",
            code("yew_agent::utils::store"),
            ", you may switch to third party solutions like: ",
            link!("https://github.com/intendednull/yewdux", "Yewdux",),
            " or ",
            link!("https://github.com/futursolo/bounce", "Bounce"),
            ".",
        ],
        h2![
            code("Threaded"),
            " has been separated into ",
            code("PublicAgent"),
            " and ",
            code("PrivateAgent"),
        ],
        p![
            "Replace ",
            code("use yew_agent::Threaded;"),
            " with ",
            code("use yew_agent::PublicAgent;"),
            ".",
        ],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                code("Threaded"),
                " was never implemented for Private Agents. All existing web worker-based agents \
                 are Public Agents.",
            ],
        ],
    ])
}

crate::doc_page!(
    "From 0.1.0 to 0.2.0",
    "/docs/migration-guides/yew-agent/from-0-1-0-to-0-2-0",
    page_content()
);
