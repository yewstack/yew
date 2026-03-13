pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![
            text("A function component is considered "),
            link(
                "https://en.wikipedia.org/wiki/Pure_function",
                vec![text("pure")],
            ),
            text(" when the returned "),
            code("Html"),
            text(
                " is deterministically derived from its props when its view function does not \
                 mutate its state or has other side effects.",
            ),
        ]),
        p(vec![
            text("The example below is a pure component. For a given prop "),
            code("is_loading"),
            text(" it will always result in the same "),
            code("Html"),
            text(" without any side effects."),
        ]),
        code_block(
            "rust",
            r#"use yew::{Properties, component, Html, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[component]
fn HelloWorld(props: &Props) -> Html {
    if props.is_loading {
        html! { "Loading" }
    } else {
        html! { "Hello world" }
    }
}"#,
        ),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text(
                    "If you have an internal pure component that makes no use of hooks and other \
                     component machinery, you can often write it instead as a normal function \
                     returning ",
                ),
                code("Html"),
                text(
                    " and avoid a bit of overhead for Yew, related to running the component \
                     lifecycle. Use ",
                ),
                link(
                    "/docs/concepts/html/literals-and-expressions#expressions",
                    vec![text("expression syntax")],
                ),
                text(" to render them in "),
                code("html!"),
                text("."),
            ])],
        ),
        h2(vec![text("Impure components")]),
        p(vec![
            text(
                "You might wonder if a component can be impure if it does not use any globals, \
                 since it is just a function that is called every render. This is where the next \
                 topic comes in - ",
            ),
            link(
                "/docs/concepts/function-components/hooks",
                vec![text("hooks")],
            ),
        ]),
    ])
}

crate::doc_page!(
    "Pure Components",
    "/docs/concepts/function-components/pure-components",
    page_content()
);
