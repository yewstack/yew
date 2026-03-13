pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![
            text("The "),
            code("html!"),
            text(
                " macro always requires a single root node. To get around this restriction, you \
                 can use an \"empty tag\" (these are also called \"fragments\").",
            ),
        ]),
        tabs(
            "Valid",
            vec![
                tab(
                    "Valid",
                    "Valid",
                    vec![code_block(
                        "rust",
                        r#"use yew::prelude::*;

html! {
    <>
        <div></div>
        <p></p>
    </>
};"#,
                    )],
                ),
                tab(
                    "Invalid",
                    "Invalid",
                    vec![code_block_compile_fail(
                        "rust",
                        r#"use yew::prelude::*;

// error: only one root html element allowed

html! {
    <div></div>
    <p></p>
};"#,
                    )],
                ),
            ],
        ),
    ])
}

crate::doc_page!("Fragments", "/docs/concepts/html/fragments", page_content());
