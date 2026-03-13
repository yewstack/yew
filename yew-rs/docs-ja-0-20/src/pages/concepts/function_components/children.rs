crate::doc_page!(
    "Children",
    "/ja/docs/concepts/function-components/children",
    Content::new(vec![
        p(vec![
            code("Children"),
            text(" is a special prop type that allows you to receive nested "),
            code("Html"),
            text(" that is provided like html child elements."),
        ]),
        code_block(
            "rust",
            r#"use yew::{function_component, html, Html, Properties, Children};

#[function_component(App)]
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

#[function_component(HelloWorld)]
fn HelloWorld(props: &Props) -> Html {
    html! {
        <div class="very-stylized-container">
    // highlight-next-line
            { for props.children.iter() } // you can forward children like this
        </div>
    }
}"#
        ),
        h2(vec![text("Further reading")]),
        ul(vec![li(vec![link(
            "/ja/docs/advanced-topics/children",
            vec![text("Advanced ways to handle children")]
        ),]),]),
    ])
);
