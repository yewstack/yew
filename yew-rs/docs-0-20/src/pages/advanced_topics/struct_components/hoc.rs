crate::doc_page!(
    "Higher Order Components",
    "/docs/advanced-topics/struct-components/hoc",
    Content::new(vec![
        p![text(
            "There are several cases where Struct components dont directly support a feature (ex. \
             Suspense) or require a lot of boiler plate to use the features (ex. Context)."
        ),],
        p![text(
            "In those cases it is recommended to create function components that are higher order \
             components."
        ),],
        h2![text("Higher Order Components Definition")],
        p![text(
            "Higher Order Components are components that dont add any new Html and only wrap some \
             other component to provide extra functionality."
        ),],
        h3![text("Example")],
        p![text(
            "Hook into Context and pass it down to a struct component"
        )],
        code_block(
            "rust",
            r##"use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct Theme {
    foreground: String,
    background: String,
}

#[function_component]
pub fn App() -> Html {
    let ctx = use_state(|| Theme {
        foreground: "#000000".to_owned(),
        background: "#eeeeee".to_owned(),
    });

    html! {
        <ContextProvider<Theme> context={(*ctx).clone()}>
            <ThemedButtonHOC />
        </ContextProvider<Theme>>
    }
}

// highlight-start
#[function_component]
pub fn ThemedButtonHOC() -> Html {
    let theme = use_context::<Theme>().expect("no ctx found");

    html! {<ThemedButtonStructComponent {theme} />}
}
// highlight-end

#[derive(Properties, PartialEq)]
pub struct Props {
    pub theme: Theme,
}

struct ThemedButtonStructComponent;

impl Component for ThemedButtonStructComponent {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let theme = &ctx.props().theme;
        html! {
            <button style={format!(
                    "background: {}; color: {};",
                    theme.background,
                    theme.foreground
                )}
            >
                { "Click me!" }
            </button>
        }
    }
}"##
        ),
    ])
);
