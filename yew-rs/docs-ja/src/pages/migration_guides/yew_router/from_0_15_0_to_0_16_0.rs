pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            text("The router API has been completely rewritten in "),
            code("0.16.0"),
            text("."),
        ],
        p![
            text(
                "Because it is such a radical change, there are too many things to list out here, \
                 so we highly recommend to read the updated ",
            ),
            link!("/ja/docs/concepts/router", text("router documentation")),
            text(" and adapt your app accordingly."),
        ],
    ])
}

crate::doc_page!(
    "From 0.15.0 to 0.16.0",
    "/ja/docs/migration-guides/yew-router/from-0-15-0-to-0-16-0",
    page_content()
);
