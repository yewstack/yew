pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![
            code("#[component]"),
            text(" 属性也适用于用于创建泛型组件的泛型函数。"),
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

// 之后可以像这样使用
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
    "泛型组件",
    "/zh-Hans/docs/concepts/function-components/generics",
    page_content()
);
