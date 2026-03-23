crate::doc_page!(
    "CSS",
    "/docs/more/css",
    Content::new(vec![
        p(vec![
            text("A proposal for integrated CSS support can be found here: "),
            link(
                "https://github.com/yewstack/yew/issues/533",
                vec![text("https://github.com/yewstack/yew/issues/533"),]
            ),
        ]),
        p(vec![text(
            "This contains a lot of discussion about how to best integrate CSS support into Yew."
        ),]),
        p(vec![text(
            "Currently, the approach we have adopted is to encourage developers to build many \
             systems, before adopting the most popular one."
        ),]),
        p(vec![text(
            "The community is currently developing several projects to make it easy to add styles \
             to projects. A few are given below:"
        ),]),
        h4(vec![text("Component Libraries")]),
        ul(vec![
            li(vec![
                link(
                    "https://github.com/spielrs/yew_styles",
                    vec![text("yew_styles")]
                ),
                text(" - A styling framework for Yew without any JavaScript dependencies."),
            ]),
            li(vec![
                link(
                    "https://github.com/Follpvosten/yew-mdc",
                    vec![text("yew-mdc")]
                ),
                text(" - Material Design Components."),
            ]),
            li(vec![
                link(
                    "https://github.com/AlephAlpha/muicss-yew",
                    vec![text("muicss-yew")]
                ),
                text(" - MUI CSS Components."),
            ]),
            li(vec![
                link("https://github.com/yewstack/yewtify", vec![text("Yewtify")]),
                text(" – Implements the features provided by the Vuetify framework in Yew."),
            ]),
        ]),
        h4(vec![text("Styling Solutions")]),
        ul(vec![
            li(vec![
                link(
                    "https://github.com/futursolo/stylist-rs",
                    vec![text("stylist")]
                ),
                text(" - A CSS-in-Rust styling solution for WebAssembly Applications."),
            ]),
            li(vec![
                link(
                    "https://github.com/thedodd/trunk/tree/master/examples/yew-tailwindcss",
                    vec![text("tailwind-css")]
                ),
                text(" - Tailwind Utility Classes."),
            ]),
        ]),
        admonition(
            AdmonitionType::Warning,
            Some("Contribute"),
            vec![p(vec![text(
                "If you're developing a project adding styles to Yew please submit a PR adding \
                 yourself to this list!"
            ),]),]
        ),
    ])
);
