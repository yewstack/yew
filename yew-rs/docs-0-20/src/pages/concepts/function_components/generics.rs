crate::doc_page!(
    "Generic Components",
    "/docs/concepts/function-components/generics",
    Content::new(vec![
        p(vec![
            text("The "),
            code("#[function_component]"),
            text(" attribute also works with generic functions for creating generic components."),
        ]),
        code_block(
            "rust",
            r##"use std::fmt::Display;
use yew::{function_component, html, Properties, Html};

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: PartialEq,
{
    data: T,
}

#[function_component]
pub fn MyGenericComponent<T>(props: &Props<T>) -> Html
where
    T: PartialEq + Display,
{
    html! {
        <p>
            { &props.data }
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
};"##
        ),
    ])
);
