pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![text("Let's revisit this previous statement:")],
        blockquote![p![text(
            "Yew centrally operates on the idea of keeping everything that a reusable piece of UI \
             may need in one place - rust files.",
        )]],
        p![text(
            "We will refine this statement, by introducing the concept that will define the logic \
             and presentation behavior of an application: \"components\".",
        )],
        h2![text("What are Components?")],
        p![text("Components are the building blocks of Yew.")],
        p![text("They:")],
        ul![
            li![
                text("Take arguments in form of "),
                link!(
                    "/docs/concepts/function-components/properties",
                    text("Props"),
                ),
            ],
            li![text("Can have their own state")],
            li![text("Compute pieces of HTML visible to the user (DOM)",)],
        ],
        h2![text("Two flavors of Yew Components")],
        p![text(
            "You are currently reading about function components - the recommended way to write \
             components when starting with Yew and when writing simple presentation logic.",
        )],
        p![
            text("There is a more advanced, but less accessible, way to write components - "),
            link!(
                "/docs/advanced-topics/struct-components",
                text("Struct components"),
            ),
            text(
                ". They allow very detailed control, though you will not need that level of \
                 detail most of the time.",
            ),
        ],
        h2![text("Creating function components")],
        p![
            text("To create a function component add the "),
            code("#[component]"),
            text(
                " attribute to a function. By convention, the function is named in PascalCase, \
                 like all components, to contrast its use to normal html elements inside the ",
            ),
            code("html!"),
            text(" macro."),
        ],
        code_block(
            "rust",
            r#"use yew::{component, html, Html};

#[component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

// Then somewhere else you can use the component inside `html!`
#[component]
fn App() -> Html {
    html! { <HelloWorld /> }
}"#,
        ),
        h2![text("What happens to components")],
        p![
            text(
                "When rendering, Yew will build a virtual tree of these components. It will call \
                 the view function of each (function) component to compute a virtual version \
                 (VDOM) of the DOM that you as the library user see as the ",
            ),
            code("Html"),
            text(" type. For the previous example, this would look like this:"),
        ],
        code_block(
            "xhtml",
            r#"<App>
    <HelloWorld>
        <p>"Hello world"</p>
    </HelloWorld>
</App>"#,
        ),
        p![
            text(
                "When an update is necessary, Yew will again call the view function and reconcile \
                 the new virtual DOM with its previous version and only propagate the \
                 new/changed/necessary parts to the actual DOM. This is what we call ",
            ),
            bold![text("rendering")],
            text("."),
        ],
        admonition!(
            AdmonitionType::Note,
            None,
            p![
                text("Behind the scenes, "),
                code("Html"),
                text(" is just an alias for "),
                code("VNode"),
                text(" - a virtual node."),
            ],
        ),
    ])
}

crate::doc_page!(
    "Function Components",
    "/docs/concepts/function-components",
    page_content()
);
