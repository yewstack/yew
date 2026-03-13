crate::doc_page!(
    "Server-side Rendering",
    "/zh-Hans/docs/advanced-topics/server-side-rendering",
    Content::new(vec![
        h1(vec![text("Server-side Rendering")]),
        p(vec![
            text("By default, Yew components render on the client side. When a viewer \
              visits a website, the server sends a skeleton HTML file without any actual \
              content and a WebAssembly bundle to the browser. \
              Everything is rendered on the client side by the WebAssembly \
              bundle. This is known as client-side rendering."),
        ]),
        p(vec![text("This approach works fine for most websites, with some caveats:")]),
        ol(vec![
            li(vec![
                text("Users will not be able to see anything until the entire WebAssembly \
                  bundle is downloaded and the initial render has been completed. \
                  This can result in a poor experience for users on a slow network."),
            ]),
            li(vec![
                text("Some search engines do not support dynamically rendered web content and \
                  those who do usually rank dynamic websites lower in the search results."),
            ]),
        ]),
        p(vec![text("To solve these problems, we can render our website on the server side.")]),
        h2(vec![text("How it Works")]),
        p(vec![
            text("Yew provides a "),
            code("ServerRenderer"),
            text(" to render pages on the server side."),
        ]),
        p(vec![
            text("To render Yew components on the server side, you can create a renderer \
              with "),
            code("ServerRenderer::<App>::new()"),
            text(" and call "),
            code("renderer.render().await"),
            text(" to render "),
            code("<App />"),
            text(" into a "),
            code("String"),
            text("."),
        ]),
        code_block("rust", r#"use yew::prelude::*;
use yew::ServerRenderer;

#[function_component]
fn App() -> Html {
    html! {<div>{"Hello, World!"}</div>}
}

// we use `flavor = "current_thread"` so this snippet can be tested in CI,
// where tests are run in a WASM environment. You likely want to use
// the (default) `multi_thread` favor as:
// #[tokio::main]
#[tokio::main(flavor = "current_thread")]
async fn no_main() {
    let renderer = ServerRenderer::<App>::new();

    let rendered = renderer.render().await;

    // Prints: <div>Hello, World!</div>
    println!("{}", rendered);
}"#),
        h2(vec![text("Component Lifecycle")]),
        p(vec![
            text("The recommended way of working with server-side rendering is function components."),
        ]),
        p(vec![
            text("All hooks other than "),
            code("use_effect"),
            text(" (and "),
            code("use_effect_with"),
            text(") will function normally until a component successfully renders into "),
            code("Html"),
            text(" for the first time."),
        ]),
        admonition(
            AdmonitionType::Warning,
            Some("Web APIs are not available!"),
            vec![p(vec![
                text("Web APIs such as "),
                code("web_sys"),
                text(" are not available when your component is rendering on the server side. \
                  Your application will panic if you try to use them. \
                  You should isolate logics that need Web APIs in "),
                code("use_effect"),
                text(" or "),
                code("use_effect_with"),
                text(" as effects are not executed during server-side rendering."),
            ])],
        ),
        admonition(
            AdmonitionType::Danger,
            Some("Struct Components"),
            vec![
                p(vec![
                    text("While it is possible to use Struct Components with server-side rendering, \
                      there are no clear boundaries between client-side safe logic like the "),
                    code("use_effect"),
                    text(" hook for function components and lifecycle events are invoked \
                      in a different order than the client side."),
                ]),
                p(vec![
                    text("In addition, Struct Components will continue to accept messages until all of its \
                      children are rendered and "),
                    code("destroy"),
                    text(" method is called. Developers need to \
                      make sure no messages possibly passed to components would link to logic \
                      that makes use of Web APIs."),
                ]),
                p(vec![
                    text("When designing an application with server-side rendering support, \
                      prefer function components unless you have a good reason not to."),
                ]),
            ],
        ),
        h2(vec![text("Data Fetching during Server-side Rendering")]),
        p(vec![
            text("Data fetching is one of the difficult points with server-side rendering and hydration."),
        ]),
        p(vec![
            text("Traditionally, when a component renders, it is instantly available \
              (outputs a virtual DOM to be rendered). This works fine when the \
              component does not want to fetch any data. But what happens if the component \
              wants to fetch some data during rendering?"),
        ]),
        p(vec![
            text("In the past, there was no mechanism for Yew to detect whether a component is still \
              fetching data. The data-fetching client is responsible to implement \
              a solution to detect what is being requested during the initial render and triggers \
              a second render after requests are fulfilled. The server repeats this process until \
              no more pending requests are added during a render before returning a response."),
        ]),
        p(vec![
            text("This not only wastes CPU resources by repeatedly rendering components, \
              but the data client also needs to provide a way to make the data fetched on the \
              server side available during the hydration process to make sure that the \
              virtual DOM returned by the initial render is consistent with the \
              server-side rendered DOM tree which can be hard to implement."),
        ]),
        p(vec![
            text("Yew takes a different approach by trying to solve this issue with "),
            code("<Suspense />"),
            text("."),
        ]),
        p(vec![
            text("Suspense is a special component that when used on the client side, provides a \
              way to show a fallback UI while the component is fetching \
              data (suspended) and resumes to normal UI when the data fetching completes."),
        ]),
        p(vec![
            text("When the application is rendered on the server side, Yew waits until a \
              component is no longer suspended before serializing it into the string buffer."),
        ]),
        p(vec![
            text("During the hydration process, elements within a "),
            code("<Suspense />"),
            text(" component remains dehydrated until all of its child components are no longer suspended."),
        ]),
        p(vec![
            text("With this approach, developers can build a client-agnostic, SSR-ready \
              application with data fetching with very little effort."),
        ]),
        h2(vec![text("Rendering <head> Tags")]),
        p(vec![
            text("A common need with SSR is rendering dynamic "),
            code("<head>"),
            text(" content (e.g. "),
            code("<title>"),
            text(", "),
            code("<meta>"),
            text(") so that crawlers and social previews see the right metadata on first load."),
        ]),
        p(vec![
            code("ServerRenderer"),
            text(" only renders your component tree (typically at the body of the document), \
              it has no access to "),
            code("<head>"),
            text(". Head tags must therefore be generated "),
            bold(vec![text("on the server, outside of Yew")]),
            text(", and spliced into the HTML template before it is sent to the client."),
        ]),
        p(vec![
            text("The "),
            link(
                "https://github.com/yewstack/yew/blob/master/examples/ssr_router/src/bin/ssr_router_server.rs",
                vec![text("ssr_router example")],
            ),
            text(" demonstrates this pattern: the server recognizes the \
              route from the request URL, generates the appropriate "),
            code("<title>"),
            text(" and "),
            code("<meta>"),
            text(" tags, and injects them into the Trunk-generated "),
            code("index.html"),
            text(" before "),
            code("</head>"),
            text("."),
        ]),
        admonition(
            AdmonitionType::Info,
            None,
            vec![p(vec![
                text("For a fully SSR-compatible third-party solution, use "),
                link(
                    "https://docs.rs/bounce/latest/bounce/helmet/index.html",
                    vec![text("the <Helmet/> component from Bounce")],
                ),
                text("."),
            ])],
        ),
        h2(vec![text("SSR Hydration")]),
        p(vec![
            text("Hydration is the process that connects a Yew application to the \
              server-side generated HTML file. By default, "),
            code("ServerRenderer"),
            text(" prints hydratable HTML string which includes additional information to facilitate hydration. \
              When the "),
            code("Renderer::hydrate"),
            text(" method is called, instead of starting rendering from \
              scratch, Yew will reconcile the Virtual DOM generated by the application \
              with the HTML string generated by the server renderer."),
        ]),
        admonition(
            AdmonitionType::Warning,
            None,
            vec![p(vec![
                text("To successfully hydrate an HTML representation created by the "),
                code("ServerRenderer"),
                text(", the client must produce a Virtual DOM layout that \
                  exactly matches the one used for SSR including components that do not \
                  contain any elements. If you have any component that is only useful in \
                  one implementation, you may want to use a "),
                code("PhantomComponent"),
                text(" to fill the position of the extra component."),
            ])],
        ),
        admonition(
            AdmonitionType::Warning,
            None,
            vec![p(vec![
                text("The hydration can only succeed if the real DOM matches the expected DOM \
                  after initial render of the SSR output (static HTML) by browser. If your HTML is \
                  not spec-compliant, the hydration "),
                italic(vec![text("may")]),
                text(" fail. Browsers may change the DOM structure \
                  of the incorrect HTML, causing the actual DOM to be different from the expected DOM. \
                  For example, "),
                link(
                    "https://github.com/yewstack/yew/issues/2684",
                    vec![text("if you have a <table> without a <tbody>, the browser may add a <tbody> to the DOM")],
                ),
                text("."),
            ])],
        ),
        h2(vec![text("Component Lifecycle during hydration")]),
        p(vec![
            text("During Hydration, components schedule 2 consecutive renders after it is \
              created. Any effects are called after the second render completes. \
              It is important to make sure that the render function of your \
              component is free of side effects. It should not mutate any states or trigger \
              additional renders. If your component currently mutates states or triggers \
              additional renders, move them into a "),
            code("use_effect"),
            text(" hook."),
        ]),
        p(vec![
            text("It is possible to use Struct Components with server-side rendering in \
              hydration, the view function will be called \
              multiple times before the rendered function will be called. \
              The DOM is considered as not connected until the rendered function is called, \
              you should prevent any access to rendered nodes \
              until "),
            code("rendered()"),
            text(" method is called."),
        ]),
        h2(vec![text("Example")]),
        code_block("rust", r#"use yew::prelude::*;
use yew::Renderer;

#[function_component]
fn App() -> Html {
    html! {<div>{"Hello, World!"}</div>}
}

fn main() {
    let renderer = Renderer::<App>::new();

    // hydrates everything under body element, removes trailing
    // elements (if any).
    renderer.hydrate();
}"#),
        ul(vec![
            li(vec![
                text("Example: "),
                link(
                    "https://github.com/yewstack/yew/tree/master/examples/simple_ssr",
                    vec![text("simple_ssr")],
                ),
            ]),
            li(vec![
                text("Example: "),
                link(
                    "https://github.com/yewstack/yew/tree/master/examples/ssr_router",
                    vec![text("ssr_router")],
                ),
            ]),
        ]),
        h2(vec![text("Single thread mode")]),
        p(vec![
            text("Yew supports single thread mode for server-side rendering by "),
            code("yew::LocalServerRenderer"),
            text(". This mode would work in a single thread environment like WASI."),
        ]),
        code_block("rust", r#"use yew::prelude::*;
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
        ul(vec![
            li(vec![
                text("Example: "),
                link(
                    "https://github.com/yewstack/yew/tree/master/examples/wasi_ssr_module",
                    vec![text("wasi_ssr_module")],
                ),
            ]),
        ]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text("If you are using the "),
                code("wasm32-unknown-unknown"),
                text(" target to build a SSR application, you can use the "),
                code("not_browser_env"),
                text(" feature flag to disable access of browser-specific APIs inside of Yew. This would be useful on serverless platforms like Cloudflare Worker."),
            ])],
        ),
        admonition(
            AdmonitionType::Warning,
            None,
            vec![p(vec![
                text("Server-side rendering is currently experimental. If you find a bug, please file \
                  an issue on "),
                link(
                    "https://github.com/yewstack/yew/issues/new?assignees=&labels=bug&template=bug_report.md&title=",
                    vec![text("GitHub")],
                ),
                text("."),
            ])],
        ),
    ])
);
