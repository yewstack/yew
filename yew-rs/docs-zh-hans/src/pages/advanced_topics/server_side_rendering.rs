pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h1![text("服务端渲染 (Server-Side Rendering)")],
        p![
            text("默认情况下，Yew 组件在客户端渲染。当用户访问一个网站时，服务器会发送一个骨架 HTML 文件，不包含任何实际内容，以及一个 WebAssembly 包给浏览器。所有内容都由 WebAssembly 包在客户端渲染。这被称为客户端渲染。"),
        ],
        p![
            text("这种方法对于大多数网站来说都是有效的，但有一些注意事项："),
        ],
        ol![
            li![
                text("用户在整个 WebAssembly 包下载并完成初始渲染之前将看不到任何内容。这可能会导致在缓慢的网络上用户体验不佳。"),
            ],
            li![
                text("一些搜索引擎不支持动态渲染的网页内容，而那些支持的搜索引擎通常会将动态网站排名较低。"),
            ],
        ],
        p![text("为了解决这些问题，我们可以在服务端渲染我们的网站。")],
        h2![text("工作原理")],
        p![
            text("Yew 提供了一个 "),
            code("ServerRenderer"),
            text(" 来在服务端渲染页面。"),
        ],
        p![
            text("要在服务端渲染 Yew 组件，您可以使用 "),
            code("ServerRenderer::<App>::new()"),
            text(" 创建一个渲染器，并调用 "),
            code("renderer.render().await"),
            text(" 将 "),
            code("<App />"),
            text(" 渲染为一个 "),
            code("String"),
            text("。"),
        ],
        code_block("rust", r#"use yew::prelude::*;
use yew::ServerRenderer;

#[component]
fn App() -> Html {
    html! {<div>{"Hello, World!"}</div>}
}

// 我们在使用 `flavor = "current_thread"` 以保证这个示例可以在 CI 中的 WASM 环境运行,
// 如果你希望使用多线程的话，可以使用默认的 `#[tokio::main]` 宏
#[tokio::main(flavor = "current_thread")]
async fn no_main() {
    let renderer = ServerRenderer::<App>::new();

    let rendered = renderer.render().await;

    // 打印: <div>Hello, World!</div>
    println!("{}", rendered);
}"#),
        h2![text("组件生命周期")],
        p![
            text("与客户端渲染不同，组件的生命周期在服务端渲染时会有所不同。"),
        ],
        p![
            text("在组件成功第一次渲染为 "),
            code("Html"),
            text(" 之前，除了 "),
            code("use_effect"),
            text(" (和 "),
            code("use_effect_with"),
            text(") 之外的所有钩子都会正常工作。"),
        ],
        admonition![
            AdmonitionType::Caution,
            Some("浏览器接口不可用！"),
            p![
                text("浏览器相关的接口，如 "),
                code("web_sys"),
                text("，在组件在服务端渲染时是不可用的。如果您尝试使用它们，您的应用程序将会崩溃。您应该将需要这部分逻辑隔离在 "),
                code("use_effect"),
                text(" 或 "),
                code("use_effect_with"),
                text(" 中，因为在服务端渲染时它们无法也不应当执行。"),
            ],
        ],
        admonition![
            AdmonitionType::Danger,
            Some("结构化组件"),
            p![
                text("尽管可以在服务端渲染时使用结构化组件，但是在客户端安全逻辑（如函数组件的 "),
                code("use_effect"),
                text(" 钩子）和生命周期事件之间没有明确的边界，并且生命周期事件的调用顺序与客户端不同。"),
            ],
            p![
                text("此外，结构化组件将继续接受消息，直到所有子组件都被渲染并调用了 "),
                code("destroy"),
                text(" 方法。开发人员需要确保不会将可能传递给组件的消息链接到调用浏览器接口的逻辑。"),
            ],
            p![
                text("在设计支持服务端渲染的应用程序时，请尽量使用函数组件，除非您有充分的理由不这样做。"),
            ],
        ],
        h2![text("服务端渲染期间的数据获取")],
        p![
            text("数据获取是服务端渲染和水合（hydration）期间的难点之一。"),
        ],
        p![
            text("传统做法中，当一个组件渲染时，它会立即可用（输出一个虚拟 DOM 以进行渲染）。当组件不需要获取任何数据时，这种方式是有效的。但是如果组件在渲染时想要获取一些数据会发生什么呢？"),
        ],
        p![
            text("过去，Yew 没有机制来检测组件是否仍在获取数据。数据获取客户端负责实现一个解决方案，以检测在初始渲染期间请求了什么，并在请求完成后触发第二次渲染。服务器会重复这个过程，直到在返回响应之前没有在渲染期间添加更多的挂起请求。"),
        ],
        p![
            text("这不仅浪费了 CPU 资源，因为重复渲染组件，而且数据客户端还需要提供一种方法，在水合过程中使在服务端获取的数据可用，以确保初始渲染返回的虚拟 DOM 与服务端渲染的 DOM 树一致，这可能很难实现。"),
        ],
        p![
            text("Yew 采用了一种不同的方法，通过 "),
            code("<Suspense />"),
            text(" 来解决这个问题。"),
        ],
        p![
            code("<Suspense />"),
            text(" 是一个特殊的组件，当在客户端使用时，它提供了一种在组件获取数据（挂起）时显示一个回退 UI 的方法，并在数据获取完成后恢复到正常 UI。"),
        ],
        p![
            text("当应用程序在服务端渲染时，Yew 会等待组件不再挂起，然后将其序列化到字符串缓冲区中。"),
        ],
        p![
            text("在水合过程中，"),
            code("<Suspense />"),
            text(" 组件中的元素保持未水合状态，直到所有子组件不再挂起。"),
        ],
        p![
            text("通过这种方法，开发人员可以轻松构建一个准备好进行服务端渲染的、与客户端无关的应用程序，并进行数据获取。"),
        ],
        h2![text("渲染 "), code("<head>"), text(" 标签")],
        p![
            text("SSR 中的一个常见需求是渲染动态 "),
            code("<head>"),
            text(" 内容（例如 "),
            code("<title>"),
            text("、"),
            code("<meta>"),
            text("），使爬虫和社交预览在首次加载时能看到正确的元数据。"),
        ],
        p![
            code("ServerRenderer"),
            text(" 只渲染组件树（通常对应文档的 body 部分），无法访问 "),
            code("<head>"),
            text("。因此，head 标签必须"),
            bold![text("在服务端、Yew 之外")],
            text("生成，并在发送给客户端之前拼接到 HTML 模板中。"),
        ],
        p![
            link!["https://github.com/yewstack/yew/blob/master/examples/ssr_router/src/bin/ssr_router_server.rs", code("ssr_router"), text(" 示例")],
            text(" 演示了这一模式：服务端从请求 URL 识别路由，生成适当的 "),
            code("<title>"),
            text(" 和 "),
            code("<meta>"),
            text(" 标签，并将它们注入到 Trunk 生成的 "),
            code("index.html"),
            text(" 的 "),
            code("</head>"),
            text(" 之前。"),
        ],
        admonition![
            AdmonitionType::Info,
            None,
            p![
                text("如需完全兼容 SSR 的第三方解决方案，请使用 "),
                link!["https://docs.rs/bounce/latest/bounce/helmet/index.html", text("Bounce 的 "), code("<Helmet/>"), text(" 组件")],
                text("。"),
            ],
        ],
        h2![text("SSR 水合（SSR Hydration）")],
        p![
            text("水合是将 Yew 应用程序连接到服务端生成的 HTML 文件的过程。默认情况下，"),
            code("ServerRender"),
            text(" 打印可水合的 HTML 字符串，其中包含额外的信息以便于水合。当调用 "),
            code("Renderer::hydrate"),
            text(" 方法时，Yew 不会从头开始渲染，而是将应用程序生成的虚拟 DOM 与服务器渲染器生成的 HTML 字符串进行协调。"),
        ],
        admonition![
            AdmonitionType::Caution,
            None,
            p![
                text("要成功对由 "),
                code("ServerRenderer"),
                text(" 创建的 HTML 标记进行水合，客户端必须生成一个虚拟 DOM 布局，它与用于 SSR 的布局完全匹配，包括不包含任何元素的组件。如果您有任何只在一个实现中有用的组件，您可能希望使用 "),
                code("PhantomComponent"),
                text(" 来填充额外组件的位置。"),
            ],
        ],
        admonition![
            AdmonitionType::Warning,
            None,
            p![
                text("只有在浏览器初始渲染 SSR 输出（静态 HTML）后，真实 DOM 与预期 DOM 匹配时，水合才能成功。如果您的 HTML 不符合规范，水合可能会失败。浏览器可能会更改不正确的 HTML 的 DOM 结构，导致实际 DOM 与预期 DOM 不同。例如，"),
                link!["https://github.com/yewstack/yew/issues/2684", text("如果您有一个没有 "), code("<tbody>"), text(" 的 "), code("<table>"), text("，浏览器可能会向 DOM 添加一个 "), code("<tbody>")],
            ],
        ],
        h2![text("水合期间的组件生命周期")],
        p![
            text("在水合期间，组件在创建后安排了 2 次连续的渲染。任何效果都是在第二次渲染完成后调用的。确保您的组件的渲染函数没有副作用是很重要的。它不应该改变任何状态或触发额外的渲染。如果您的组件当前改变状态或触发额外的渲染，请将它们移动到 "),
            code("use_effect"),
            text(" 钩子中。"),
        ],
        p![
            text("在水合过程中，可以使用结构化组件进行服务端渲染，视图函数将在渲染函数之前被调用多次。直到调用渲染函数之前，DOM 被认为是未连接的，您应该防止在调用 "),
            code("rendered()"),
            text(" 方法之前访问渲染节点。"),
        ],
        h2![text("示例")],
        code_block_ignore("rust", r#"use yew::prelude::*;
use yew::Renderer;

#[component]
fn App() -> Html {
    html! {<div>{"Hello, World!"}</div>}
}

fn main() {
    let renderer = Renderer::<App>::new();

    // 对 body 元素下的所有内容进行水合，并移除尾部元素（如果有）。
    renderer.hydrate();
}"#),
        p![
            text("示例: "),
            link!["https://github.com/yewstack/yew/tree/master/examples/simple_ssr", text("simple_ssr")],
        ],
        p![
            text("示例: "),
            link!["https://github.com/yewstack/yew/tree/master/examples/ssr_router", text("ssr_router")],
        ],
        h2![text("单线程模式")],
        p![
            text("Yew 支持以单线程进行服务端渲染，通过 "),
            code("yew::LocalServerRenderer"),
            text("。这种模式适用于像 WASI 这样的单线程环境。"),
        ],
        code_block("rust", r#"// 使用 `wasm32-wasip1` 或 `wasm32-wasip2` 目标构建。

use yew::prelude::*;
use yew::LocalServerRenderer;

#[component]
fn App() -> Html {
    use yew_router::prelude::*;

    html! {
        <>
            <h1>{"Yew WASI SSR demo"}</h1>
        </>
    }
}

pub async fn render() -> String {
    let renderer = LocalServerRenderer::<App>::new();
    let html_raw = renderer.render().await;

    let mut body = String::new();
    body.push_str("<body>");
    body.push_str("<div id='app'>");
    body.push_str(&html_raw);
    body.push_str("</div>");
    body.push_str("</body>");

    body
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("{}", render().await);
}"#),
        p![
            text("示例: "),
            link!["https://github.com/yewstack/yew/tree/master/examples/wasi_ssr_module", text("wasi_ssr_module")],
        ],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                text("如果您使用 "),
                code("wasm32-unknown-unknown"),
                text(" 目标构建 SSR 应用程序，您可以使用 "),
                code("not_browser_env"),
                text(" 功能标志来禁用 Yew 内部对特定于浏览器的 API 的访问。这在像 Cloudflare Worker 这样的无服务器平台上非常有用。"),
            ],
        ],
        admonition![
            AdmonitionType::Caution,
            None,
            p![
                text("服务端渲染目前是实验性的。如果您发现了一个 bug，"),
                link!["https://github.com/yewstack/yew/issues/new?assignees=&labels=bug&template=bug_report.md&title=", text("请在 GitHub 反馈")],
                text("。"),
            ],
        ],
    ])
}

crate::doc_page!(
    "服务端渲染",
    "/zh-Hans/docs/advanced-topics/server-side-rendering",
    page_content()
);
