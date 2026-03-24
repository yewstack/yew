pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![text(
            "在某些情況下，結構組件不直接支援某些功能（例如 \
             Suspense），或使用某些功能需要大量的樣板程式碼（例如 Context）。",
        )],
        p![text("在這些情況下，建議建立高階組件的函數組件。")],
        h2![text("高階組件定義")],
        p![text(
            "高階元件是不添加任何新 HTML 的元件，只是包裝其他元件以提供額外功能。",
        )],
        h3![text("範例")],
        p![text("對 Context (上下文) 掛鉤並將其傳遞給結構組件")],
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
}



"##,
        ),
    ])
}

crate::doc_page!(
    "高階組件",
    "/zh-Hant/docs/advanced-topics/struct-components/hoc",
    page_content()
);
