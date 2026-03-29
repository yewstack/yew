pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "Yew does not ship built-in CSS support, but the community maintains several styling \
             solutions. Below are actively maintained projects you can use today."
        ],
        h4!["Styling Solutions"],
        ul![
            li![
                link!["https://github.com/futursolo/stylist-rs", "stylist",],
                " - A CSS-in-Rust styling solution.",
            ],
            li![
                link![
                    "https://github.com/thedodd/trunk/tree/master/examples/yew-tailwindcss",
                    "tailwind-css",
                ],
                " - Tailwind Utility Classes.",
            ],
        ],
        admonition![
            AdmonitionType::Important,
            Some("contribute"),
            p![
                "If you're developing a project adding styles to Yew please submit a PR adding \
                 yourself to this list!"
            ],
        ],
        hr(),
        h4!["Inactive Projects"],
        p![
            "The projects below are no longer actively maintained but may still serve as useful \
             references for learning or as starting points for new efforts. If you're interested \
             in reviving or continuing any of them, contributions are welcome!"
        ],
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
                " – Implements the features provided by the Vuetify framework in Yew.",
            ],
        ],
    ])
}

crate::doc_page!("CSS", "/docs/more/css", page_content());
