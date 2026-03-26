crate::doc_page!(
    "Children",
    "/zh-Hant/docs/concepts/function-components/children",
    Content::new(vec![
        p![
            code("Children"),
            " is a special prop type that allows you to receive nested ",
            code("Html"),
            " that is provided like html child elements.",
        ],
        code_block(
            "rust",
            r#"use yew::{function_component, html, Html, Properties, Children};

#[function_component]
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
    pub children: Children, // the field name `children` is important!
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! {
        <div class="very-stylized-container">
    // highlight-next-line
            { for props.children.iter() } // you can forward children like this
        </div>
    }
}"#
        ),
        h2!["Further reading"],
        ul![li![link!(
            "/zh-Hant/docs/advanced-topics/children",
            "Advanced ways to handle children"
        )]],
    ])
);
