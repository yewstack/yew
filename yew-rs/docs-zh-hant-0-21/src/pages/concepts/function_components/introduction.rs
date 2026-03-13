crate::doc_page!(
    "Function Components",
    "/zh-Hant/docs/concepts/function-components",
    Content::new(vec![
        p(vec![text("Let's revisit this previous statement:")]),
        blockquote(vec![p(vec![text(
            "Yew centrally operates on the idea of keeping everything that a reusable piece of UI \
             may need in one place - rust files.",
        )])]),
        p(vec![text(
            "We will refine this statement, by introducing the concept that will define the logic \
             and presentation behavior of an application: \"components\".",
        )]),
        h2(vec![text("What are Components?")]),
        p(vec![text("Components are the building blocks of Yew.")]),
        p(vec![text("They:")]),
        ul(vec![
            li(vec![
                text("Take arguments in form of "),
                link("", vec![text("Props")]),
            ]),
            li(vec![text("Can have their own state")]),
            li(vec![text(
                "Compute pieces of HTML visible to the user (DOM)",
            )]),
        ]),
        h2(vec![text("Two flavors of Yew Components")]),
        p(vec![text(
            "You are currently reading about function components - the recommended way to write \
             components when starting with Yew and when writing simple presentation logic.",
        )]),
        p(vec![
            text("There is a more advanced, but less accessible, way to write components - ",),
            link(
                "advanced-topics/struct-components/introduction.mdx",
                vec![text("Struct components")],
            ),
            text(
                ". They allow very detailed control, though you will not need that level of \
                 detail most of the time.",
            ),
        ]),
        h2(vec![text("Creating function components")]),
        p(vec![
            text("To create a function component add the "),
            code("#[function_component]"),
            text(
                " attribute to a function. By convention, the function is named in PascalCase, \
                 like all components, to contrast its use to normal html elements inside the ",
            ),
            code("html!"),
            text(" macro."),
        ]),
        code_block(
            "rust",
            r#"use yew::{function_component, html, Html};

#[function_component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

// Then somewhere else you can use the component inside `html!`
#[function_component]
fn App() -> Html {
    html! { <HelloWorld /> }
}"#,
        ),
        h2(vec![text("What happens to components")]),
        p(vec![text(
            "When rendering, Yew will build a virtual tree of these components. It will call the \
             view function of each (function) component to compute a virtual version (VDOM) of \
             the DOM that you as the library user see as the Html type. For the previous example, \
             this would look like this:",
        )]),
        code_block(
            "xhtml",
            r#"<App>
    <HelloWorld>
        <p>"Hello world"</p>
    </HelloWorld>
</App>"#,
        ),
        p(vec![
            text(
                "When an update is necessary, Yew will again call the view function and reconcile \
                 the new virtual DOM with its previous version and only propagate the \
                 new/changed/necessary parts to the actual DOM. This is what we call ",
            ),
            bold(vec![text("rendering")]),
            text("."),
        ]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text("Behind the scenes, "),
                code("Html"),
                text(" is just an alias for "),
                code("VNode"),
                text(" - a virtual node."),
            ])],
        ),
    ])
);
