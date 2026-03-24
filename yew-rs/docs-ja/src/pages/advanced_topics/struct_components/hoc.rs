pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![text(
            "いくつかの状況では、構造コンポーネントは特定の機能（例えば \
             Suspense）を直接サポートしていないか、\
             または特定の機能を使用するために大量のボイラープレートコードが必要です（例えば \
             Context）。",
        )],
        p![text(
            "このような場合、高階コンポーネントの関数コンポーネントを作成することをお勧めします。",
        )],
        h2![text("高階コンポーネントの定義")],
        p![text(
            "高階コンポーネントは、新しい HTML \
             を追加せず、他のコンポーネントをラップして追加機能を提供するコンポーネントです。",
        )],
        h3![text("例")],
        p![text(
            "Context（コンテキスト）フックを使用し、それを構造コンポーネントに渡す例",
        )],
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
    "高階コンポーネント",
    "/ja/docs/advanced-topics/struct-components/hoc",
    page_content()
);
