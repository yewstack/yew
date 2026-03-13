crate::doc_page!(
    "Components",
    "/docs/concepts/html/components",
    Content::new(vec![
        h2(vec![text("Basic")]),
        p(vec![
            text("Components can be used in the "),
            code("html!"),
            text(" macro:"),
        ]),
        code_block("rust", r#"use yew::prelude::*;

#[function_component]
fn MyComponent() -> Html {
    html! {
        { "This component has no properties!" }
    }
}

#[derive(Clone, PartialEq, Properties)]
struct Props {
    user_first_name: String,
    user_last_name: String,
}

#[function_component]
fn MyComponentWithProps(props: &Props) -> Html {
    let Props { user_first_name, user_last_name } = props;
    html! {
        <>{"user_first_name: "}{user_first_name}{" and user_last_name: "}{user_last_name}</>
    }
}

let props = Props {
    user_first_name: "Bob".to_owned(),
    user_last_name: "Smith".to_owned(),
};

html!{
    <>
        // No properties
        <MyComponent />

        // With Properties
        <MyComponentWithProps user_first_name="Sam" user_last_name="Idle" />

        // With the whole set of props provided at once
        <MyComponentWithProps ..props.clone() />

        // With Properties from a variable and specific values overridden
        <MyComponentWithProps user_last_name="Elm" ..props />
    </>
};"#),
        h2(vec![text("Nested")]),
        p(vec![
            text("Components can accept child components/elements if they have a "),
            code("children"),
            text(" field in their "),
            code("Properties"),
        ]),
        code_block_title("rust", "parent.rs", r#"use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct Props {
    id: String,
    children: Html,
}

#[function_component]
fn Container(props: &Props) -> Html {
    html! {
        <div id={props.id.clone()}>
            { props.children.clone() }
        </div>
    }
}

html! {
    <Container id="container">
        <h4>{ "Hi" }</h4>
        <div>{ "Hello" }</div>
    </Container>
};"#),
        p(vec![
            text("The "),
            code("html!"),
            text(" macro allows you to pass a base expression with the "),
            code("..props"),
            text(" syntax instead of specifying each property individually, \
                  similar to Rust's "),
            link(
                "https://doc.rust-lang.org/stable/reference/expressions/struct-expr.html#functional-update-syntax",
                vec![text("Functional Update Syntax")]
            ),
            text(". This base expression must occur after any individual props are passed. \
                  When passing a base props expression with a "),
            code("children"),
            text(" field, the children passed in the "),
            code("html!"),
            text(" macro overwrite the ones already present in the props."),
        ]),
        code_block("rust", r#"use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct Props {
    id: String,
    children: Html,
}

#[function_component]
fn Container(props: &Props) -> Html {
    html! {
        <div id={props.id.clone()}>
            { props.children.clone() }
        </div>
    }
}

let props = yew::props!(Props {
    id: "container-2",
    children: Html::default(),
});

html! {
    <Container ..props>
        // props.children will be overwritten with this
        <span>{ "I am a child, as you can see" }</span>
    </Container>
};"#),
        h2(vec![text("Relevant examples")]),
        ul(vec![
            li(vec![link("https://github.com/yewstack/yew/tree/master/examples/function_todomvc", vec![text("Function Todo MVC")])]),
            li(vec![link("https://github.com/yewstack/yew/tree/master/examples/function_router", vec![text("Function Router")])]),
        ]),
    ])
);
