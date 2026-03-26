pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "The Yew repository contains many ",
            link!(
                "https://github.com/yewstack/yew/tree/master/examples",
                "examples",
            ),
            " (in various states of maintenance). We recommend perusing them to get a feel for \
             how to use different features of the framework. We also welcome Pull Requests and \
             issues for when they inevitably get neglected and need some ❤️",
        ],
        p![
            "For more details including a list of examples, refer to the ",
            link!(
                "https://github.com/yewstack/yew/tree/master/examples#yew-examples",
                "README",
            ),
            ".",
        ],
        admonition!(
            AdmonitionType::Tip,
            None,
            p![
                "Most of the examples have a live deployment that can be found at ",
                code("https://examples.yew.rs/< example_name >"),
                ". Click the shield on their README page in their respective sub-folder to \
                 navigate to the live demo.",
            ],
        ),
    ])
}

crate::doc_page!("Examples", "/docs/getting-started/examples", page_content());
