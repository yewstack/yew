crate::doc_page!(
    "Fragments",
    "/zh-Hans/docs/concepts/html/fragments",
    Content::new(vec![
        p![
            "The ",
            code("html!"),
            " macro always requires a single root node. In order to get around this restriction, \
             you can use an \"empty tag\" (these are also called \"fragments\").",
        ],
        p!["Valid:"],
        code_block(
            "rust",
            r#"use yew::prelude::*;

html! {
    <>
        <div></div>
        <p></p>
    </>
};"#
        ),
        p!["Invalid:"],
        code_block(
            "rust",
            r#"use yew::prelude::*;

// error: only one root html element allowed

html! {
    <div></div>
    <p></p>
};"#
        ),
    ])
);
