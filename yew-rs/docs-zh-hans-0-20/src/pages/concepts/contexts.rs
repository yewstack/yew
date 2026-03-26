crate::doc_page!(
    "Contexts",
    "/zh-Hans/docs/concepts/contexts",
    Content::new(vec![
        p![
            "Usually, data is passed from a parent component to a child component via props. \
              But passing props can become verbose and annoying if you have to pass them through many components in the middle, \
              or if many components in your app need the same information. Context solve this problem by allowing a \
              parent component to make data available to any component in the tree below it, no matter how deep, \
              without having to pass it down with props.",
        ],
        h2!["The problem with props: \"Prop Drilling\""],
        p![
            "Passing ",
            link!["/docs/0.20/concepts/function-components/properties", "props"],
            " is a great way to pass data directly from parent to a child. \
              They become cumbersome to pass down through deeply nested component tree or when multiple components share the same data. \
              A common solution to data sharing is lifting the data to a common ancestor and making the children take it as props. \
              However, this can lead to cases where the prop has to go through multiple components in order to reach the component needs it. \
              This situation is called \"Prop Drilling\".",
        ],
        p!["Consider the following example which passes down the theme using props:"],
        code_block("rust", r##"use yew::{html, Children, Component, Context, Html, Properties, function_component};

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
    children: Children,
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
        p![
            "We \"drill\" the theme prop through ",
            code("Navbar"),
            " so that it can reach ",
            code("Title"),
            " and ",
            code("NavButton"),
            ". It would be nice if ",
            code("Title"),
            " and ",
            code("NavButton"),
            ", the components that need access to the theme, can just access the theme \
              without having to pass it to them as prop. Contexts solve this problem by allowing a parent to pass data, theme in this case, \
              to its children.",
        ],
        h2!["Using Contexts"],
        h3!["Step 1: Providing the context"],
        p![
            "A context provider is required to consume the context. ",
            code("ContextProvider<T>"),
            ", where ",
            code("T"),
            " is the context struct is used as the provider. ",
            code("T"),
            " must implement ",
            code("Clone"),
            " and ",
            code("PartialEq"),
            ". ",
            code("ContextProvider"),
            " is the component whose children will have the context available to them. \
              The children are re-rendered when the context changes. A struct is used to define what data is to be passed. The ",
            code("ContextProvider"),
            " can be used as:",
        ],
        code_block("rust", r##"use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct Theme {
    foreground: String,
    background: String,
}

#[function_component]
fn NavButton() -> Html {
    let theme = use_context::<Rc<Theme>>().expect("Context not found");

    html! {
        // use theme
    }
}

#[function_component]
fn App() -> Html {
    let theme = use_memo(|_| Theme {
        foreground: "yellow".to_owned(),
        background: "pink".to_owned(),
    }, ());

    html! {
        <ContextProvider<Rc<Theme>> context={theme}>
            <NavButton />
        </ContextProvider<Rc<Theme>>>
    }
}"##),
        h3!["Step 2: Consuming context"],
        h4!["Function components"],
        p![
            code("use_context"),
            " hook is used to consume contexts in function components. \
              See ",
            link!["https://yew-rs-api.web.app/next/yew/functional/fn.use_context.html", "docs for use_context"],
            " to learn more.",
        ],
        h4!["Struct components"],
        p!["We have 2 options to consume contexts in struct components:"],
        ul![
            li![
                link!["/docs/0.20/advanced-topics/struct-components/hoc", "Higher Order Components"],
                ": A higher order function component will consume the context and pass the data to the struct component which requires it.",
            ],
            li![
                "Consume context directly in struct component. See ",
                link!["https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/contexts/src/struct_component_subscriber.rs", "example of struct component as a consumer"],
            ],
        ],
        h2!["Use cases"],
        p![
            "Generally, if some data is needed by distant components in different parts of the tree, it's likely that context will help you. \
              Here's some examples of such cases:",
        ],
        ul![
            li![
                bold!["Theming"],
                ": You can put a context at the top of the app that holds your app theme and use it to adjust the visual appearance, as shown in the above example.",
            ],
            li![
                bold!["Current user account"],
                ": In many cases, components need to know the current logged-in user. You can use a context to provide the current user object to the components.",
            ],
        ],
        h3!["Considerations to make before using contexts"],
        p![
            "Contexts are very easy to use. That makes them very easy to misuse/overuse. \
              Just because you can use a context to share props to components multiple levels deep, doesn't mean that you should.",
        ],
        p![
            "For example, you may be able to extract a component and pass that component as a child to another component. For example, \
              you may have a ",
            code("Layout"),
            " component which takes ",
            code("articles"),
            " as prop and passes it down to ",
            code("ArticleList"),
            " component. You should refactor the ",
            code("Layout"),
            " component to take children as props and display ",
            code("<Layout> <ArticleList {articles} /> </Layout>"),
            ".",
        ],
        h2!["Mutating context value a child"],
        p![
            "Because of Rust's ownership rules, a context cannot have a method that takes ",
            code("&mut self"),
            " that can be called by children. In order to mutate a context's value, we must combine it with a reducer. This is done by using the ",
            link!["https://yew-rs-api.web.app/next/yew/functional/fn.use_reducer.html", code("use_reducer")],
            " hook.",
        ],
        p![
            "The ",
            link!["https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/contexts", "contexts example"],
            " demonstrates mutable contexts with the help of contexts",
        ],
        h2!["Further reading"],
        ul![
            li![
                "The ",
                link!["https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/contexts", "contexts example"],
            ],
        ],
    ])
);
