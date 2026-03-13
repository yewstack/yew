pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![
            text("The "),
            code("#[component]"),
            text(" attribute also works with generic functions for creating generic components."),
        ]),
        code_block(
            "rust",
            r#"use std::fmt::Display;
use yew::{component, html, Properties, Html};

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: PartialEq,
{
    data: T,
}

#[component]
pub fn MyGenericComponent<T>(props: &Props<T>) -> Html
where
    T: PartialEq + Clone + Into<Html>,
{
    html! {
        <p>
            { props.data.clone().into() }
        </p>
    }
}

// then can be used like this
html! {
    <MyGenericComponent<i32> data=123 />
};

// or
html! {
    <MyGenericComponent<String> data={"foo".to_string()} />
};"#,
        ),
    ])
}

crate::doc_page!(
    "Generic Components",
    "/docs/concepts/function-components/generics",
    page_content()
);
