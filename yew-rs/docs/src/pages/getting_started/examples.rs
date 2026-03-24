pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            text("The Yew repository contains many "),
            link!(
                "https://github.com/yewstack/yew/tree/master/examples",
                text("examples"),
            ),
            text(
                " (in various states of maintenance). We recommend perusing them to get a feel \
                 for how to use different features of the framework. We also welcome Pull \
                 Requests and issues for when they inevitably get neglected and need some ❤️",
            ),
        ],
        p![
            text("For more details including a list of examples, refer to the "),
            link!(
                "https://github.com/yewstack/yew/tree/master/examples#yew-examples",
                text("README"),
            ),
            text("."),
        ],
        admonition!(
            AdmonitionType::Tip,
            None,
            p![
                text("Most of the examples have a live deployment that can be found at "),
                code("https://examples.yew.rs/< example_name >"),
                text(
                    ". Click the shield on their README page in their respective sub-folder to \
                     navigate to the live demo.",
                ),
            ],
        ),
    ])
}

crate::doc_page!("Examples", "/docs/getting-started/examples", page_content());
