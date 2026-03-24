pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            code("Children"),
            text(" is a special prop type that allows you to receive nested "),
            code("Html"),
            text(" that is provided like html child elements."),
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
    pub children: Html, // the field name `children` is important!
}

#[component]
fn HelloWorld(props: &Props) -> Html {
    html! {
        <div class="very-stylized-container">
    // highlight-next-line
            { props.children.clone() } // you can forward children like this
        </div>
    }
}"#,
        ),
    ])
}

crate::doc_page!(
    "Children",
    "/docs/concepts/function-components/children",
    page_content()
);
