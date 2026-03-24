crate::doc_page!(
    "CSS",
    "/docs/more/css",
    Content::new(vec![
        p![
            text("A proposal for integrated CSS support can be found here: "),
            link!(
                "https://github.com/yewstack/yew/issues/533",
                text("https://github.com/yewstack/yew/issues/533"),
            ),
        ],
        p![text(
            "This contains a lot of discussion about how to best integrate CSS support into Yew."
        ),],
        p![text(
            "Currently, the approach we have adopted is to encourage developers to build many \
             systems, before adopting the most popular one."
        ),],
        p![text(
            "The community is currently developing several projects to make it easy to add styles \
             to projects. A few are given below:"
        ),],
        h4![text("Component Libraries")],
        ul![
            li![
                link!(
                    "https://github.com/spielrs/yew_styles",
                    text("yew_styles"),
                ),
                text(" - A styling framework for Yew without any JavaScript dependencies."),
            ],
            li![
                link!(
                    "https://github.com/Follpvosten/yew-mdc",
                    text("yew-mdc"),
                ),
                text(" - Material Design Components."),
            ],
            li![
                link!(
                    "https://github.com/AlephAlpha/muicss-yew",
                    text("muicss-yew"),
                ),
                text(" - MUI CSS Components."),
            ],
            li![
                link!("https://github.com/yewstack/yewtify", text("Yewtify")),
                text(" – Implements the features provided by the Vuetify framework in Yew."),
            ],
        ],
        h4![text("Styling Solutions")],
        ul![
            li![
                link!(
                    "https://github.com/futursolo/stylist-rs",
                    text("stylist"),
                ),
                text(" - A CSS-in-Rust styling solution for WebAssembly Applications."),
            ],
            li![
                link!(
                    "https://github.com/thedodd/trunk/tree/master/examples/yew-tailwindcss",
                    text("tailwind-css"),
                ),
                text(" - Tailwind Utility Classes."),
            ],
        ],
        admonition![
            AdmonitionType::Warning,
            Some("Contribute"),
            p![text(
                "If you're developing a project adding styles to Yew please submit a PR adding \
                 yourself to this list!"
            ),],
        ],
    ])
);
