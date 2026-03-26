pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p!["通常，資料是透過 props 從父元件傳遞到子元件。 \
             但是，如果必須透過中間的許多元件傳遞它們，或者如果應用程式中的許多元件需要相同的訊息，傳遞 props 可能會變得冗長和煩人。 \
             上下文解決了這個問題，允許父元件使資料可用於其下方樹中的任何元件，無論多深，而無需透過 props 傳遞它們。"],
        h2!["使用 props 的問題：\"Prop Drilling\""],
        p![
            "傳遞 ",
            doc_link!(crate::pages::concepts::function_components::properties, "props"),
            " 是從父元件直接傳遞資料到子元件的好方法。 \
                 但是，當需要透過深層嵌套的組件樹傳遞資料或多個組件共享相同的資料時，傳遞 props 變得繁瑣。 \
                 一種常見的資料共享解決方案是將資料提升到一個共同的祖先，並使子元件將其作為 props 接收。 \
                 然而，這可能導致 props 必須通過多個元件才能到達需要它的元件。 \
                 這種情況稱為 \"Prop Drilling\"。",
        ],
        p!["考慮以下範例，它透過 props 傳遞主題："],
        code_block(
            "rust",
            r#"use yew::{html, Component, Context, Html, Properties, component};

#[derive(Clone, PartialEq)]
pub struct Theme {
    foreground: String,
    background: String,
}

#[derive(PartialEq, Properties)]
pub struct NavbarProps {
    theme: Theme,
}

#[component]
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

#[component]
fn Title(_props: &ThemeProps) -> Html {
    html! {
        // impl
    }
}

#[component]
fn NavButton(_props: &ThemeProps) -> Html {
    html! {
        // impl
    }
}

/// App 根節點
#[component]
fn App() -> Html {
    let theme = Theme {
        foreground: "yellow".to_owned(),
        background: "pink".to_owned(),
    };

    html! {
        <Navbar {theme} />
    }
}"#,
        ),
        p![
            "我們透過 ",
            code("Navbar"),
            " 傳遞主題設定，以便它可以到達 ",
            code("Title"),
            " 和 ",
            code("NavButton"),
            "。 如果 ",
            code("Title"),
            " 和 ",
            code("NavButton"),
            " 這些需要存取主題的元件可以直接存取主題而不必透過 prop 傳遞，那就更好了。 上下文解決了這個問題，允許父元件將資料（在這種情況下是主題）傳遞給其子元件。",
        ],
        h2!["使用上下文"],
        h3!["步驟 1：提供上下文"],
        p![
            "需要一個上下文提供者來消費上下文。 ",
            code("ContextProvider<T>"),
            "，其中 ",
            code("T"),
            " 是用作提供者的上下文結構體。 ",
            code("T"),
            " 必須實作 ",
            code("Clone"),
            " 和 ",
            code("PartialEq"),
            "。 ",
            code("ContextProvider"),
            " 是其子元件將擁有上下文的元件。 當上下文變更時，子元件會重新渲染。一個結構體用來定義要傳遞的資料。 ",
            code("ContextProvider"),
            " 可以這樣使用：",
        ],
        code_block(
            "rust",
            r##"use yew::prelude::*;

/// App 主題
#[derive(Clone, Debug, PartialEq)]
struct Theme {
    foreground: String,
    background: String,
}

/// 主組件
#[component]
pub fn App() -> Html {
    let ctx = use_state(|| Theme {
        foreground: "#000000".to_owned(),
        background: "#eeeeee".to_owned(),
    });

    html! {
        // `ctx` 是 `Rc<UseStateHandle<Theme>>` 類型，而我們需要 `Theme`
        // 所以我們對它進行解引用。
        <ContextProvider<Theme> context={(*ctx).clone()}>
            // 這裡的每個子元件及其子元件都將存取此上下文。
            <Toolbar />
        </ContextProvider<Theme>>
    }
}

/// 工具栏
/// 此组件可以访问上下文。
#[component]
pub fn Toolbar() -> Html {
    html! {
        <div>
            <ThemedButton />
        </div>
    }
}

/// 放置在 `Toolbar` 中的按鈕
/// 由於此元件是元件樹中 `ThemeContextProvider` 的子元件，它也可以存取上下文。
#[component]
pub fn ThemedButton() -> Html {
    let theme = use_context::<Theme>().expect("no ctx found");

    html! {
        <button style={format!("background: {}; color: {};", theme.background, theme.foreground)}>
            { "Click me!" }
        </button>
    }
}"##,
        ),
        h3!["步驟 2：使用上下文"],
        h4!["函數元件"],
        p![
            code("use_context"),
            " 鉤子用於在函數元件中使用上下文。 請參閱 ",
            link!(
                "https://yew-rs-api.web.app/next/yew/functional/fn.use_context.html",
                "use_context 文件",
            ),
            " 以了解更多資訊。",
        ],
        h4!["結構體組件"],
        p!["我們有兩種選擇在結構體組件中使用上下文："],
        ul![
            li![
                doc_link!(crate::pages::advanced_topics::struct_components::hoc, "高階元件"),
                "：高階函數元件將使用上下文並將資料傳遞給需要它的結構體元件。",
            ],
            li![
                "直接在結構體組件中使用上下文。請參閱 ",
                link!(
                    "https://github.com/yewstack/yew/tree/master/examples/contexts/src/struct_component_subscriber.rs",
                    "結構體組件作\u{200B}\u{200B}為消費者的範例",
                ),
            ],
        ],
        h2!["使用場景"],
        p!["通常，如果某些資料需要在樹的不同部分的遠端元件中使用，上下文可能會對你有所幫助。 以下是一些這樣的例子："],
        ul![
            li![
                bold!["主題"],
                "：你可以在應用程式的頂部放置一個上下文來保存你的應用程式主題，並使用它來調整視覺外觀，如上例所示。",
            ],
            li![
                bold!["目前使用者帳戶"],
                "：在許多情況下，元件需要知道目前登入的使用者。你可以使用上下文將目前使用者物件提供給元件。",
            ],
        ],
        h3!["使用上下文前的考慮"],
        p!["上下文非常容易使用，這也使得它們非常容易被誤用/過度使用。 \
             僅僅因為你可以使用上下文將 props 共享給多個層級深的元件，並不意味著你應該這樣做。"],
        p![
            "例如，你可以提取一個元件並將該元件作為子元件傳遞給另一個元件。例如， 你可能有一個 ",
            code("Layout"),
            " 元件，它將 ",
            code("articles"),
            " 作為 prop 並傳遞給 ",
            code("ArticleList"),
            " 元件。 你應該重構 ",
            code("Layout"),
            " 元件，使其接受子元件作為 props 並顯示 ",
            code("<Layout> <ArticleList {articles} /> </Layout>"),
            "。",
        ],
        h2!["修改子元件的上下文值"],
        p![
            "由於 Rust 的所有權規則，上下文不能有一個可以被子元件呼叫的 ",
            code("&mut self"),
            " 方法。 要修改上下文的值，我們必須將其與 reducer 結合使用。這可以透過使用 ",
            link!(
                "https://yew-rs-api.web.app/next/yew/functional/fn.use_reducer.html",
                code("use_reducer"),
            ),
            " 鉤子完成。",
        ],
        p![
            link!(
                "https://github.com/yewstack/yew/tree/master/examples/contexts",
                "上下文範例",
            ),
            " 示範了使用上下文的可變上下文",
        ],
        h2!["進一步閱讀"],
        ul![li![link!(
            "https://github.com/yewstack/yew/tree/master/examples/contexts",
            "上下文範例",
        )]],
    ])
}

crate::doc_page!(
    "上下文 (Contexts)",
    "/zh-Hant/docs/concepts/contexts",
    page_content()
);
