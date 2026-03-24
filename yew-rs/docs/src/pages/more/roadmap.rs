pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("Prioritization")],
        p![
            text(
                "The prioritization of upcoming features and focuses of the framework is \
                 determined by the community. In Spring 2020, a developer survey was sent out to \
                 collect feedback on the direction of the project. You can find the summary in \
                 the ",
            ),
            link![
                "https://github.com/yewstack/yew/wiki/Dev-Survey-%5BSpring-2020%5D",
                text("Yew Wiki"),
            ],
            text("."),
        ],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                text("Status of all major initiatives can be tracked on the Yew Github "),
                link![
                    "https://github.com/yewstack/yew/projects",
                    text("project board"),
                ],
            ],
        ],
        h2![text("Focuses")],
        ol![
            li![text("Top Requested Features")],
            li![text("Production Readiness")],
            li![text("Documentation")],
            li![text("Pain Points")],
        ],
        h3![text("Most requested features")],
        ol![
            li![link![
                "https://github.com/yewstack/yew/projects/3",
                text("Functional Components"),
            ]],
            li![link![
                "https://github.com/yewstack/yew/projects/4",
                text("Component Library"),
            ]],
            li![text("Better state management")],
            li![link![
                "https://github.com/yewstack/yew/projects/5",
                text("Server side rendering"),
            ]],
        ],
        h3![text("Issues needed for production readiness")],
        ul![
            li![text("Improve Yew test coverage")],
            li![text("Reduce binary size")],
            li![link![
                "https://github.com/yewstack/yew/issues/5",
                text("Benchmark performance"),
            ]],
        ],
        h3![text("Documentation")],
        ul![
            li![text("Create tutorial")],
            li![text("Simplify project setup")],
        ],
        h3![text("Pain points")],
        ul![
            li![link![
                "https://github.com/yewstack/yew/issues/830",
                text("Component boilerplate"),
            ]],
            li![link![
                "https://github.com/yewstack/yew/projects/6",
                text("Agents"),
            ]],
        ],
    ])
}

crate::doc_page!("Roadmap", "/docs/more/roadmap", page_content());
