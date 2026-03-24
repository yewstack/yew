pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            code("#[component]"),
            text(" 屬性也適用於用於建立泛型元件的泛型函數。"),
        ],
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

// 之後可以這樣使用
html! {
    <MyGenericComponent<i32> data=123 />
};

// 或者
html! {
    <MyGenericComponent<String> data={"foo".to_string()} />
};"#,
        ),
    ])
}

crate::doc_page!(
    "泛型元件",
    "/zh-Hant/docs/concepts/function-components/generics",
    page_content()
);
