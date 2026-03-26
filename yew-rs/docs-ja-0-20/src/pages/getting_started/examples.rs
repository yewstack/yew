crate::doc_page!(
    "Examples",
    "/ja/docs/getting-started/examples",
    Content::new(vec![
        p![
            "The Yew repository contains many ",
            link!(
                "https://github.com/yewstack/yew/tree/yew-v0.20.0/examples",
                "examples"
            ),
            " (in various states of maintenance). We recommend perusing them to get a feel for \
             how to use different features of the framework. We also welcome Pull Requests and \
             issues for when they inevitably get neglected and need some love.",
        ],
        p![
            "For more details including a list of examples, refer to the ",
            link!(
                "https://github.com/yewstack/yew/tree/yew-v0.20.0/examples#yew-examples",
                "README"
            ),
            ".",
        ],
        admonition![
            AdmonitionType::Tip,
            None,
            p![
                "Most of the examples have a live deployment that can be found at ",
                code("https://examples.yew.rs/< example_name >"),
                ". Click the shield on their individual README page in their respective \
                 sub-folder to navigate to the live demo.",
            ],
        ],
    ])
);
