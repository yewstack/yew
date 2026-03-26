pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p!["在一些情况下，结构组件不直接支持某些功能（例如 \
            Suspense），或者使用某些功能需要大量的样板代码（例如 Context）。"],
        p!["在这些情况下，建议创建高阶组件的函数组件。"],
        h2!["高阶组件定义"],
        p!["高阶组件是不添加任何新 HTML 的组件，只是包装其他组件以提供额外功能。"],
        h3!["示例"],
        p!["对 Context (上下文) 挂钩并将其传递给结构组件"],
        code_block(
            "rust",
            r##"use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct Theme {
    foreground: String,
    background: String,
}

#[component]
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
#[component]
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
}"##,
        ),
    ])
}

crate::doc_page!(
    "高阶组件",
    "/zh-Hans/docs/advanced-topics/struct-components/hoc",
    page_content()
);
