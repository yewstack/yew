pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("Prioritization")]),
        p(vec![
            text(
                "The prioritization of upcoming features and focuses of the framework is \
                 determined by the community. In Spring 2020, a developer survey was sent out to \
                 collect feedback on the direction of the project. You can find the summary in \
                 the ",
            ),
            link(
                "https://github.com/yewstack/yew/wiki/Dev-Survey-%5BSpring-2020%5D",
                vec![text("Yew Wiki")],
            ),
            text("."),
        ]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text("Status of all major initiatives can be tracked on the Yew Github "),
                link(
                    "https://github.com/yewstack/yew/projects",
                    vec![text("project board")],
                ),
            ])],
        ),
        h2(vec![text("Focuses")]),
        ol(vec![
            li(vec![text("Top Requested Features")]),
            li(vec![text("Production Readiness")]),
            li(vec![text("Documentation")]),
            li(vec![text("Pain Points")]),
        ]),
        h3(vec![text("Most requested features")]),
        ol(vec![
            li(vec![link(
                "https://github.com/yewstack/yew/projects/3",
                vec![text("Functional Components")],
            )]),
            li(vec![link(
                "https://github.com/yewstack/yew/projects/4",
                vec![text("Component Library")],
            )]),
            li(vec![text("Better state management")]),
            li(vec![link(
                "https://github.com/yewstack/yew/projects/5",
                vec![text("Server side rendering")],
            )]),
        ]),
        h3(vec![text("Issues needed for production readiness")]),
        ul(vec![
            li(vec![text("Improve Yew test coverage")]),
            li(vec![text("Reduce binary size")]),
            li(vec![link(
                "https://github.com/yewstack/yew/issues/5",
                vec![text("Benchmark performance")],
            )]),
        ]),
        h3(vec![text("Documentation")]),
        ul(vec![
            li(vec![text("Create tutorial")]),
            li(vec![text("Simplify project setup")]),
        ]),
        h3(vec![text("Pain points")]),
        ul(vec![
            li(vec![link(
                "https://github.com/yewstack/yew/issues/830",
                vec![text("Component boilerplate")],
            )]),
            li(vec![link(
                "https://github.com/yewstack/yew/projects/6",
                vec![text("Agents")],
            )]),
        ]),
    ])
}

crate::doc_page!("Roadmap", "/docs/more/roadmap", page_content());
