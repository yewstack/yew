pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["Prioritization"],
        p![
            "The prioritization of upcoming features and focuses of the framework is determined \
             by the community. In Spring 2020, a developer survey was sent out to collect \
             feedback on the direction of the project. You can find the summary in the ",
            link![
                "https://github.com/yewstack/yew/wiki/Dev-Survey-%5BSpring-2020%5D",
                "Yew Wiki",
            ],
            ".",
        ],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                "Status of all major initiatives can be tracked on the Yew Github ",
                link!["https://github.com/yewstack/yew/projects", "project board",],
            ],
        ],
        h2!["Focuses"],
        ol![
            li!["Top Requested Features"],
            li!["Production Readiness"],
            li!["Documentation"],
            li!["Pain Points"],
        ],
        h3!["Most requested features"],
        ol![
            li![link![
                "https://github.com/yewstack/yew/projects/3",
                "Functional Components",
            ]],
            li![link![
                "https://github.com/yewstack/yew/projects/4",
                "Component Library",
            ]],
            li!["Better state management"],
            li![link![
                "https://github.com/yewstack/yew/projects/5",
                "Server side rendering",
            ]],
        ],
        h3!["Issues needed for production readiness"],
        ul![
            li!["Improve Yew test coverage"],
            li!["Reduce binary size"],
            li![link![
                "https://github.com/yewstack/yew/issues/5",
                "Benchmark performance",
            ]],
        ],
        h3!["Documentation"],
        ul![li!["Create tutorial"], li!["Simplify project setup"],],
        h3!["Pain points"],
        ul![
            li![link![
                "https://github.com/yewstack/yew/issues/830",
                "Component boilerplate",
            ]],
            li![link![
                "https://github.com/yewstack/yew/projects/6",
                "Agents",
            ]],
        ],
    ])
}

crate::doc_page!("Roadmap", "/docs/more/roadmap", page_content());
