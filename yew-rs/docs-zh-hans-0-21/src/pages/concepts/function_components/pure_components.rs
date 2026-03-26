crate::doc_page!(
    "Pure Components",
    "/zh-Hans/docs/concepts/function-components/pure-components",
    Content::new(vec![
        p![
            "A function component is considered ",
            link!("https://en.wikipedia.org/wiki/Pure_function", "pure"),
            " when the returned ",
            code("Html"),
            " is deterministically derived from its props when its view function does not mutate \
             its state or has other side effects."
        ],
        p![
            "The example below is a pure component. For a given prop ",
            code("is_loading"),
            " it will always result in the same ",
            code("Html"),
            " without any side effects."
        ],
        code_block(
            "rust",
            r#"use yew::{Properties, function_component, Html, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    if props.is_loading {
        html! { "Loading" }
    } else {
        html! { "Hello world" }
    }
}"#
        ),
        admonition!(
            AdmonitionType::Note,
            None,
            p![
                "If you have an internal pure component that makes no use of hooks and other \
                 component machinery, you can often write it instead as a normal function \
                 returning ",
                code("Html"),
                " and avoid a bit of overhead for Yew, related to running the component \
                 lifecycle. Use expression syntax to render them in ",
                code("html!"),
                "."
            ]
        ),
        h2!["Impure components"],
        p![
            "You might wonder if a component can be impure if it does not use any globals, since \
             it is just a function that is called every render. This is where the next topic \
             comes in - ",
            link!("/docs/0.21/concepts/function-components/hooks", "hooks")
        ]
    ])
);
