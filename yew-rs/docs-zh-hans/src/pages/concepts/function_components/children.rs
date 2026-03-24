pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            code("Children"),
            text(" 是一种特殊的属性类型，允许您接收嵌套的 "),
            code("Html"),
            text("，就像 html 子元素一样提供。"),
        ],
        code_block(
            "rust",
            r#"use yew::{component, html, Html, Properties};

#[component]
fn App() -> Html {
    html! {
        // highlight-start
        <HelloWorld>
            <span>{"Hey what is up ;)"}</span>
            <h1>{"THE SKY"}</h1>
        </HelloWorld>
        // highlight-end
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    // highlight-next-line
    pub children: Html, // `children` 键很重要！
}

#[component]
fn HelloWorld(props: &Props) -> Html {
    html! {
        <div class="very-stylized-container">
    // highlight-next-line
            { props.children.clone() } // 可以靠这种方式转发子元素
        </div>
    }
}"#,
        ),
    ])
}

crate::doc_page!(
    "子元素 (Children)",
    "/zh-Hans/docs/concepts/function-components/children",
    page_content()
);
