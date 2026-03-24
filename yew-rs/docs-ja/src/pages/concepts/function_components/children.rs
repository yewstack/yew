pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            code("Children"),
            text(" は特別なプロパティタイプで、HTMLの子要素のようにネストされた "),
            code("Html"),
            text(" を受け取ることができます。"),
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
    pub children: Html, // `children` キーは重要です！
}

#[component]
fn HelloWorld(props: &Props) -> Html {
    html! {
        <div class="very-stylized-container">
    // highlight-next-line
            { props.children.clone() } // この方法で子要素を転送できます
        </div>
    }
}"#,
        ),
    ])
}

crate::doc_page!(
    "子要素 (Children)",
    "/ja/docs/concepts/function-components/children",
    page_content()
);
