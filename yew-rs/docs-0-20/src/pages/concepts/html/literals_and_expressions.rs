crate::doc_page!(
    "Literals and Expressions",
    "/docs/concepts/html/literals-and-expressions",
    Content::new(vec![
        h2(vec![text("Literals")]),
        p(vec![
            text("If expressions resolve to types that implement "),
            code("Display"),
            text(", they will be converted to strings and inserted into the DOM as a "),
            link(
                "https://developer.mozilla.org/en-US/docs/Web/API/Text",
                vec![text("Text")]
            ),
            text(" node."),
        ]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text("String literals create "),
                code("Text"),
                text(
                    " nodes, which are treated as strings by the browser. Hence, even if the \
                     expression contains a "
                ),
                code("<script>"),
                text(
                    " tag you can't fall for XSS and such security issues, unless of course you \
                     wrap the expression in a "
                ),
                code("<script>"),
                text(" block."),
            ]),]
        ),
        p(vec![
            text("All display text must be enclosed by "),
            code("{}"),
            text(
                " blocks because text is handled as an expression. This is the largest deviation \
                 from normal HTML syntax that Yew makes."
            ),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

let text = "lorem ipsum";
html!{
<>
<div>{text}</div>
<div>{"dolor sit"}</div>
<span>{42}</span>
</>
};"#
        ),
        h2(vec![text("Expressions")]),
        p(vec![
            text("You can insert expressions in your HTML using "),
            code("{}"),
            text(" blocks, as long as they resolve to "),
            code("Html"),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

let show_link = true;

html! {
<div>
{
if show_link {
html! {
  <a href="https://example.com">{"Link"}</a>
}
} else {
html! {}
}
}
</div>
};"#
        ),
        p(vec![text(
            "It often makes sense to extract these expressions into functions or closures to \
             optimize for readability:"
        )]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

let show_link = true;
let maybe_display_link = move || -> Html {
if show_link {
html! {
<a href="https://example.com">{"Link"}</a>
}
} else {
html! {}
}
};

html! {
<div>{maybe_display_link()}</div>
};"#
        ),
    ])
);
