pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["Literals"],
        p![
            "If expressions resolve to types that implement ",
            code("Display"),
            ", they will be converted to strings and inserted into the DOM as a ",
            link!(
                "https://developer.mozilla.org/en-US/docs/Web/API/Text",
                "Text",
            ),
            " node.",
        ],
        admonition!(
            AdmonitionType::Note,
            None,
            p![
                "String literals create ",
                code("Text"),
                " nodes, which are treated as strings by the browser. Hence, even if the \
                 expression contains a ",
                code("<script>"),
                " tag you can't fall for XSS and such security issues, unless of course you wrap \
                 the expression in a ",
                code("<script>"),
                " block.",
            ],
        ),
        p![
            "All display text must be enclosed by ",
            code("{}"),
            " blocks because the text is handled as an expression. This is the largest deviation \
             from normal HTML syntax that Yew makes.",
        ],
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
};"#,
        ),
        h2!["Expressions"],
        p![
            "You can insert expressions in your HTML using ",
            code("{}"),
            " blocks, as long as they resolve to ",
            code("Html"),
        ],
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
};"#,
        ),
        p![
            "It often makes sense to extract these expressions into functions or closures to \
             optimize for readability:"
        ],
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
};"#,
        ),
    ])
}

crate::doc_page!(
    "Literals and Expressions",
    "/docs/concepts/html/literals-and-expressions",
    page_content()
);
