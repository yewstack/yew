crate::doc_page!(
    "Contexts",
    "/docs/concepts/contexts",
    Content::new(vec![
        p(vec![
            text("Usually, data is passed from a parent component to a child component via props. \
                  But passing props can become verbose and annoying if you have to pass them through many components in the middle, \
                  or if many components in your app need the same information. Context solves this problem by allowing a \
                  parent component to make data available to "),
            italic(vec![text("any")]),
            text(" component in the tree below it, no matter how deep, \
                  without having to pass it down with props."),
        ]),
        h2(vec![text("The problem with props: \"Prop Drilling\"")]),
        p(vec![
            text("Passing "),
            link("/docs/concepts/function-components/properties", vec![text("props")]),
            text(" is a great way to pass data directly from a parent to a child. \
                  They become cumbersome to pass down through deeply nested component trees or when multiple components share the same data. \
                  A common solution to data sharing is lifting the data to a common ancestor and making the children take it as props. \
                  However, this can lead to cases where the prop has to go through multiple components to reach the component that needs it. \
                  This situation is called \"Prop Drilling\"."),
        ]),
        p(vec![text("Consider the following example which passes down the theme using props:")]),
        code_block("rust", r##"use yew::{html, Component, Context, Html, Properties, function_component};

#[derive(Clone, PartialEq)]
pub struct Theme {
    foreground: String,
    background: String,
}

#[derive(PartialEq, Properties)]
pub struct NavbarProps {
    theme: Theme,
}

#[function_component]
fn Navbar(props: &NavbarProps) -> Html {
    html! {
        <div>
            <Title theme={props.theme.clone()}>
                { "App title" }
            </Title>
            <NavButton theme={props.theme.clone()}>
                { "Somewhere" }
            </NavButton>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct ThemeProps {
    theme: Theme,
    children: Html,
}

#[function_component]
fn Title(_props: &ThemeProps) -> Html {
    html! {
        // impl
    }
}

#[function_component]
fn NavButton(_props: &ThemeProps) -> Html {
    html! {
        // impl
    }
}

/// App root
#[function_component]
fn App() -> Html {
    let theme = Theme {
        foreground: "yellow".to_owned(),
        background: "pink".to_owned(),
    };

    html! {
        <Navbar {theme} />
    }
}"##),
        p(vec![
            text("We \"drill\" the theme prop through "),
            code("Navbar"),
            text(" so that it can reach "),
            code("Title"),
            text(" and "),
            code("NavButton"),
            text(". It would be nice if "),
            code("Title"),
            text(" and "),
            code("NavButton"),
            text(", the components that need access to the theme, can just access the theme \
                  without having to pass it to them as a prop. Contexts solve this problem by allowing a parent to pass data, theme in this case, \
                  to its children."),
        ]),
        h2(vec![text("Using Contexts")]),
        h3(vec![text("Step 1: Providing the context")]),
        p(vec![
            text("A context provider is required to consume the context. "),
            code("ContextProvider<T>"),
            text(", where "),
            code("T"),
            text(" is the context struct used as the provider. "),
            code("T"),
            text(" must implement "),
            code("Clone"),
            text(" and "),
            code("PartialEq"),
            text(". "),
            code("ContextProvider"),
            text(" is the component whose children will have the context available to them. \
                  The children are re-rendered when the context changes. A struct is used to define what data is to be passed. The "),
            code("ContextProvider"),
            text(" can be used as:"),
        ]),
        code_block("rust", r##"use yew::prelude::*;


/// App theme
#[derive(Clone, Debug, PartialEq)]
struct Theme {
    foreground: String,
    background: String,
}

/// Main component
#[function_component]
pub fn App() -> Html {
    let ctx = use_state(|| Theme {
        foreground: "#000000".to_owned(),
        background: "#eeeeee".to_owned(),
    });

    html! {
        // `ctx` is type `Rc<UseStateHandle<Theme>>` while we need `Theme`
        // so we deref it.
        // It derefs to `&Theme`, hence the clone
        <ContextProvider<Theme> context={(*ctx).clone()}>
            // Every child here and their children will have access to this context.
            <Toolbar />
        </ContextProvider<Theme>>
    }
}

/// The toolbar.
/// This component has access to the context
#[function_component]
pub fn Toolbar() -> Html {
    html! {
        <div>
            <ThemedButton />
        </div>
    }
}

/// Button placed in `Toolbar`.
/// As this component is a child of `ThemeContextProvider` in the component tree, it also has access
/// to the context.
#[function_component]
pub fn ThemedButton() -> Html {
    let theme = use_context::<Theme>().expect("no ctx found");

    html! {
        <button style={format!("background: {}; color: {};", theme.background, theme.foreground)}>
            { "Click me!" }
        </button>
    }
}"##),
        h3(vec![text("Step 2: Consuming context")]),
        h4(vec![text("Function components")]),
        p(vec![
            code("use_context"),
            text(" hook is used to consume contexts in function components. \
                  See "),
            link(
                "https://yew-rs-api.web.app/next/yew/functional/fn.use_context.html",
                vec![text("docs for use_context")]
            ),
            text(" to learn more."),
        ]),
        h4(vec![text("Struct components")]),
        p(vec![text("We have 2 options to consume contexts in struct components:")]),
        ul(vec![
            li(vec![
                link("/docs/advanced-topics/struct-components/hoc", vec![text("Higher Order Components")]),
                text(": A higher-order function component will consume the context and pass the data to the struct component which requires it."),
            ]),
            li(vec![
                text("Consume context directly in the struct component. See "),
                link(
                    "https://github.com/yewstack/yew/tree/master/examples/contexts/src/struct_component_subscriber.rs",
                    vec![text("example of struct component as a consumer")]
                ),
            ]),
        ]),
        h2(vec![text("Use cases")]),
        p(vec![
            text("Generally, if some data is needed by distant components in different parts of the tree, context will likely help you. \
                  Here are some examples of such cases:"),
        ]),
        ul(vec![
            li(vec![
                bold(vec![text("Theming")]),
                text(": You can put a context at the top of the app that holds your app theme and use it to adjust the visual appearance, as shown in the above example."),
            ]),
            li(vec![
                bold(vec![text("Current user account")]),
                text(": In many cases, components need to know the currently logged-in user. You can use a context to provide the current user object to the components."),
            ]),
        ]),
        h3(vec![text("Considerations to make before using contexts")]),
        p(vec![
            text("Contexts are very easy to use. That makes them very easy to misuse/overuse. \
                  Just because you can use a context to share props to components multiple levels deep, does not mean that you should."),
        ]),
        p(vec![
            text("For example, you may be able to extract a component and pass that component as a child to another component. For example, \
                  you may have a "),
            code("Layout"),
            text(" component that takes "),
            code("articles"),
            text(" as a prop and passes it down to "),
            code("ArticleList"),
            text(" component. You should refactor the "),
            code("Layout"),
            text(" component to take children as props and display "),
            code("<Layout> <ArticleList {articles} /> </Layout>"),
            text("."),
        ]),
        h2(vec![text("Mutating the context value of a child")]),
        p(vec![
            text("Because of Rust's ownership rules, a context cannot have a method that takes "),
            code("&mut self"),
            text(" that can be called by children. To mutate a context's value, we must combine it with a reducer. This is done by using the "),
            link(
                "https://yew-rs-api.web.app/next/yew/functional/fn.use_reducer.html",
                vec![code("use_reducer")]
            ),
            text(" hook."),
        ]),
        p(vec![
            text("The "),
            link(
                "https://github.com/yewstack/yew/tree/master/examples/contexts",
                vec![text("contexts example")]
            ),
            text(" demonstrates mutable contexts with the help of contexts"),
        ]),
        h2(vec![text("Further reading")]),
        ul(vec![
            li(vec![
                text("The "),
                link(
                    "https://github.com/yewstack/yew/tree/master/examples/contexts",
                    vec![text("contexts example")]
                ),
            ]),
        ]),
    ])
);
