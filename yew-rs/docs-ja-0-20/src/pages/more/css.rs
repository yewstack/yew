crate::doc_page!(
    "CSS",
    "/ja/docs/more/css",
    Content::new(vec![
        p![
            "A proposal for integrated CSS support can be found here: ",
            link![
                "https://github.com/yewstack/yew/issues/533",
                "https://github.com/yewstack/yew/issues/533",
            ],
        ],
        p!["This contains a lot of discussion about how to best integrate CSS support into Yew.",],
        p![
            "Currently, the approach we've adopted is to encourage developers to build a number \
             of systems, before adopting the most popular one.",
        ],
        p![
            "The community are currently developing a number of projects to make it easy to add \
             styles to projects. A few are given below:",
        ],
        h4!["Component Libraries"],
        ul![
            li![
                link!["https://github.com/spielrs/yew_styles", "yew_styles",],
                " - A styling framework for Yew without any JavaScript dependencies.",
            ],
            li![
                link!["https://github.com/Follpvosten/yew-mdc", "yew-mdc",],
                " - Material Design Components.",
            ],
            li![
                link!["https://github.com/AlephAlpha/muicss-yew", "muicss-yew",],
                " - MUI CSS Components.",
            ],
            li![
                link!["https://github.com/yewstack/yewtify", "Yewtify"],
                " - Implements the features provided by the Vuetify framework in Yew.",
            ],
        ],
        h4!["Styling Solutions"],
        ul![li![
            link!["https://github.com/futursolo/stylist-rs", "stylist",],
            " - A CSS-in-Rust styling solution for WebAssembly Applications.",
        ],],
        admonition![
            AdmonitionType::Important,
            Some("Contribute"),
            p![
                "If you're developing a project adding styles to Yew please submit a PR adding \
                 yourself to this list!",
            ],
        ],
    ])
);
