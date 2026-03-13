crate::doc_page!(
    "Conditional Rendering",
    "/ja/docs/concepts/html/conditional-rendering",
    Content::new(vec![
        h2(vec![text("If blocks")]),
        p(vec![
            text("To conditionally render some markup, we wrap it in an "),
            code("if"),
            text(" block:")
        ]),
        p(vec![code("if"), text(":")]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

html! {
if true {
<p>{ "True case" }</p>
}
};"#
        ),
        p(vec![code("if - else"), text(":")]),
        code_block(
            "rust",
            r#"use yew::prelude::*;
let some_condition = true;

html! {
if some_condition {
<p>{ "True case" }</p>
} else {
<p>{ "False case" }</p>
}
};"#
        ),
        p(vec![code("if let"), text(":")]),
        code_block(
            "rust",
            r#"use yew::prelude::*;
let some_text = Some("text");

html! {
if let Some(text) = some_text {
<p>{ text }</p>
}
};"#
        ),
        p(vec![code("if let else"), text(":")]),
        code_block(
            "rust",
            r#"use yew::prelude::*;
let some_text = Some("text");

html! {
if let Some(text) = some_text {
<p>{ text }</p>
} else {
<p>{ "False case" }</p>
}
};"#
        ),
    ])
);
