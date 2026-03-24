pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![text(
            "通常，数据是通过 props 从父组件传递到子组件。 \
             但是，如果必须通过中间的许多组件传递它们，或者如果应用程序中的许多组件需要相同的信息，传递 \
             props 可能会变得冗长和烦人。 \
             上下文解决了这个问题，允许父组件使数据可用于其下方树中的任何组件，无论多深，而无需通过 \
             props 传递它们。",
        )],
        h2![text("使用 props 的问题：\"Prop Drilling\"")],
        p![
            text("传递 "),
            link!("/zh-Hans/docs/concepts/function-components/properties", text("props")),
            text(
                " 是从父组件直接传递数据到子组件的好方法。 \
                 但是，当需要通过深层嵌套的组件树传递数据或多个组件共享相同的数据时，传递 props 变得繁琐。 \
                 一种常见的数据共享解决方案是将数据提升到一个共同的祖先，并使子组件将其作为 props 接收。 \
                 然而，这可能导致 props 必须通过多个组件才能到达需要它的组件。 \
                 这种情况称为 \"Prop Drilling\"。",
            ),
        ],
        p![text("考虑以下示例，它通过 props 传递主题：")],
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

/// App 根节点
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
            text("我们通过 "),
            code("Navbar"),
            text(" 传递主题设定，以便它可以到达 "),
            code("Title"),
            text(" 和 "),
            code("NavButton"),
            text("。 如果 "),
            code("Title"),
            text(" 和 "),
            code("NavButton"),
            text(
                " 这些需要访问主题的组件可以直接访问主题而不必通过 prop 传递，那就更好了。 \
                 上下文解决了这个问题，允许父组件将数据（在这种情况下是主题）传递给其子组件。",
            ),
        ],
        h2![text("使用上下文")],
        h3![text("步骤 1：提供上下文")],
        p![
            text("需要一个上下文提供者来消费上下文。"),
            code("ContextProvider<T>"),
            text("，其中 "),
            code("T"),
            text(" 是用作提供者的上下文结构体。 "),
            code("T"),
            text(" 必须实现 "),
            code("Clone"),
            text(" 和 "),
            code("PartialEq"),
            text("。"),
            code("ContextProvider"),
            text(
                " 是其子组件将拥有上下文的组件。 \
                 当上下文更改时，子组件会重新渲染。一个结构体用于定义要传递的数据。",
            ),
            code("ContextProvider"),
            text(" 可以这样使用："),
        ],
        code_block(
            "rust",
            r##"use yew::prelude::*;

/// App 主题
#[derive(Clone, Debug, PartialEq)]
struct Theme {
    foreground: String,
    background: String,
}

/// 主组件
#[component]
pub fn App() -> Html {
    let ctx = use_state(|| Theme {
        foreground: "#000000".to_owned(),
        background: "#eeeeee".to_owned(),
    });

    html! {
        // `ctx` 是 `Rc<UseStateHandle<Theme>>` 类型，而我们需要 `Theme`
        // 所以我们对它进行解引用。
        <ContextProvider<Theme> context={(*ctx).clone()}>
            // 这里的每个子组件及其子组件都将访问此上下文。
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

/// 放置在 `Toolbar` 中的按钮
/// 由于此组件是组件树中 `ThemeContextProvider` 的子组件，它也可以访问上下文。
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
        h3![text("步骤 2：使用上下文")],
        h4![text("函数组件")],
        p![
            code("use_context"),
            text(" 钩子用于在函数组件中使用上下文。 请参阅 "),
            link!(
                "https://yew-rs-api.web.app/next/yew/functional/fn.use_context.html",
                text("use_context 文档"),
            ),
            text(" 了解更多信息。"),
        ],
        h4![text("结构体组件")],
        p![text("我们有两种选择在结构体组件中使用上下文：")],
        ul![
            li![
                link!("/zh-Hans/docs/advanced-topics/struct-components/hoc", text("高阶组件")),
                text(
                    "：一个高阶函数组件将使用上下文并将数据传递给需要它的结构体组件。",
                ),
            ],
            li![
                text("直接在结构体组件中使用上下文。请参阅 "),
                link!(
                    "https://github.com/yewstack/yew/tree/master/examples/contexts/src/struct_component_subscriber.rs",
                    text("结构体组件作为消费者的示例"),
                ),
            ],
        ],
        h2![text("使用场景")],
        p![text(
            "通常，如果某些数据需要在树的不同部分的远程组件中使用，上下文可能会对你有所帮助。 \
             以下是一些这样的例子：",
        )],
        ul![
            li![
                bold![text("主题")],
                text(
                    "：你可以在应用程序的顶部放置一个上下文来保存你的应用程序主题，并使用它来调整视觉外观，如上例所示。",
                ),
            ],
            li![
                bold![text("当前用户帐户")],
                text(
                    "：在许多情况下，组件需要知道当前登录的用户。你可以使用上下文将当前用户对象提供给组件。",
                ),
            ],
        ],
        h3![text("使用上下文前的考虑")],
        p![text(
            "上下文非常容易使用，这也使得它们非常容易被误用/过度使用。 \
             仅仅因为你可以使用上下文将 props \
             共享给多个层级深的组件，并不意味着你应该这样做。",
        )],
        p![
            text("例如，你可以提取一个组件并将该组件作为子组件传递给另一个组件。例如， 你可能有一个 "),
            code("Layout"),
            text(" 组件，它将 "),
            code("articles"),
            text(" 作为 prop 并传递给 "),
            code("ArticleList"),
            text(" 组件。 你应该重构 "),
            code("Layout"),
            text(" 组件，使其接受子组件作为 props 并显示 "),
            code("<Layout> <ArticleList {articles} /> </Layout>"),
            text("。"),
        ],
        h2![text("修改子组件的上下文值")],
        p![
            text("由于 Rust 的所有权规则，上下文不能有一个可以被子组件调用的 "),
            code("&mut self"),
            text(" 方法。 要修改上下文的值，我们必须将其与 reducer 结合使用。这可以通过使用 "),
            link!(
                "https://yew-rs-api.web.app/next/yew/functional/fn.use_reducer.html",
                code("use_reducer"),
            ),
            text(" 钩子来完成。"),
        ],
        p![
            link!(
                "https://github.com/yewstack/yew/tree/master/examples/contexts",
                text("上下文示例"),
            ),
            text(" 演示了使用上下文的可变上下文"),
        ],
        h2![text("进一步阅读")],
        ul![li![link!(
            "https://github.com/yewstack/yew/tree/master/examples/contexts",
            text("上下文示例"),
        )]],
    ])
}

crate::doc_page!(
    "上下文 (Contexts)",
    "/zh-Hans/docs/concepts/contexts",
    page_content()
);
