crate::doc_page!("Server-side Rendering", "/ja/docs/advanced-topics/server-side-rendering",
    Content::new(vec![
        h1![text("Server-side Rendering")],
        p![
            text("By default, Yew components render on the client side. When a viewer \
              visits a website, the server sends a skeleton HTML file without any actual \
              content and a WebAssembly bundle to the browser. \
              Everything is rendered on the client side by the WebAssembly \
              bundle. This is known as client-side rendering."),
        ],
        p![text("This approach works fine for most websites, with some caveats:")],
        ol![
            li![
                text("Users will not be able to see anything until the entire WebAssembly \
                  bundle is downloaded and the initial render has been completed. \
                  This can result in a poor experience for users on a slow network."),
            ],
            li![
                text("Some search engines do not support dynamically rendered web content and \
                  those who do usually rank dynamic websites lower in the search results."),
            ],
        ],
        p![text("To solve these problems, we can render our website on the server side.")],
        h2![text("How it Works")],
        p![
            text("Yew provides a "),
            code("ServerRenderer"),
            text(" to render pages on the server side."),
        ],
        p![
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
        ],
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
        h2![text("Component Lifecycle")],
        p![
            text("The recommended way of working with server-side rendering is function components."),
        ],
        p![
            text("All hooks other than "),
            code("use_effect"),
            text(" (and "),
            code("use_effect_with"),
            text(") will function normally until a component successfully renders into "),
            code("Html"),
            text(" for the first time."),
        ],
        admonition![
            AdmonitionType::Caution,
            Some("Web APIs are not available!"),
            p![
                text("Web APIs such as "),
                code("web_sys"),
                text(" are not available when your component is rendering on the server side. \
                  Your application will panic if you try to use them. \
                  You should isolate logics that need Web APIs in "),
                code("use_effect"),
                text(" or "),
                code("use_effect_with"),
                text(" as effects are not executed during server-side rendering."),
            ],
        ],
        admonition![
            AdmonitionType::Danger,
            Some("Struct Components"),
            p![
                text("While it is possible to use Struct Components with server-side rendering, \
                  there are no clear boundaries between client-side safe logic like the "),
                code("use_effect"),
                text(" hook for function components and lifecycle events are invoked \
                  in a different order than the client side."),
            ],
            p![
                text("In addition, Struct Components will continue to accept messages until all of its \
                  children are rendered and "),
                code("destroy"),
                text(" method is called. Developers need to \
                  make sure no messages possibly passed to components would link to logic \
                  that makes use of Web APIs."),
            ],
            p![
                text("When designing an application with server-side rendering support, \
                  prefer function components unless you have a good reason not to."),
            ],
        ],
        h2![text("Data Fetching during Server-side Rendering")],
        p![
            text("Data fetching is one of the difficult points with server-side rendering and hydration."),
        ],
        p![
            text("Traditionally, when a component renders, it is instantly available \
              (outputs a virtual DOM to be rendered). This works fine when the \
              component does not want to fetch any data. But what happens if the component \
              wants to fetch some data during rendering?"),
        ],
        p![
            text("In the past, there was no mechanism for Yew to detect whether a component is still \
              fetching data. The data-fetching client is responsible to implement \
              a solution to detect what is being requested during the initial render and triggers \
              a second render after requests are fulfilled. The server repeats this process until \
              no more pending requests are added during a render before returning a response."),
        ],
        p![
            text("This not only wastes CPU resources by repeatedly rendering components, \
              but the data client also needs to provide a way to make the data fetched on the \
              server side available during the hydration process to make sure that the \
              virtual DOM returned by the initial render is consistent with the \
              server-side rendered DOM tree which can be hard to implement."),
        ],
        p![
            text("Yew takes a different approach by trying to solve this issue with "),
            code("<Suspense />"),
            text("."),
        ],
        p![
            text("Suspense is a special component that when used on the client side, provides a \
              way to show a fallback UI while the component is fetching \
              data (suspended) and resumes to normal UI when the data fetching completes."),
        ],
        p![
            text("When the application is rendered on the server side, Yew waits until a \
              component is no longer suspended before serializing it into the string buffer."),
        ],
        p![
            text("During the hydration process, elements within a "),
            code("<Suspense />"),
            text(" component remains dehydrated until all of its child components are no longer suspended."),
        ],
        p![
            text("With this approach, developers can build a client-agnostic, SSR-ready \
              application with data fetching with very little effort."),
        ],
        h2![text("SSR Hydration")],
        p![
            text("Hydration is the process that connects a Yew application to the \
              server-side generated HTML file. By default, "),
            code("ServerRender"),
            text(" prints hydratable HTML string which includes additional information to facilitate hydration. \
              When the "),
            code("Renderer::hydrate"),
            text(" method is called, instead of starting rendering from \
              scratch, Yew will reconcile the Virtual DOM generated by the application \
              with the HTML string generated by the server renderer."),
        ],
        admonition![
            AdmonitionType::Caution,
            None,
            p![
                text("To successfully hydrate an HTML representation created by the "),
                code("ServerRenderer"),
                text(", the client must produce a Virtual DOM layout that \
                  exactly matches the one used for SSR including components that do not \
                  contain any elements. If you have any component that is only useful in \
                  one implementation, you may want to use a "),
                code("PhantomComponent"),
                text(" to fill the position of the extra component."),
            ],
        ],
        admonition![
            AdmonitionType::Warning,
            None,
            p![
                text("The hydration can only succeed if the real DOM matches the expected DOM \
                  after initial render of the SSR output (static HTML) by browser. If your HTML is \
                  not spec-compliant, the hydration "),
                italic![text("may")],
                text(" fail. Browsers may change the DOM structure \
                  of the incorrect HTML, causing the actual DOM to be different from the expected DOM. \
                  For example, "),
                link!("https://github.com/yewstack/yew/issues/2684", text("if you have a <table> without a <tbody>, the browser may add a <tbody> to the DOM")),
                text("."),
            ],
        ],
        h2![text("Component Lifecycle during hydration")],
        p![
            text("During Hydration, components schedule 2 consecutive renders after it is \
              created. Any effects are called after the second render completes. \
              It is important to make sure that the render function of your \
              component is free of side effects. It should not mutate any states or trigger \
              additional renders. If your component currently mutates states or triggers \
              additional renders, move them into a "),
            code("use_effect"),
            text(" hook."),
        ],
        p![
            text("It is possible to use Struct Components with server-side rendering in \
              hydration, the view function will be called \
              multiple times before the rendered function will be called. \
              The DOM is considered as not connected until the rendered function is called, \
              you should prevent any access to rendered nodes \
              until "),
            code("rendered()"),
            text(" method is called."),
        ],
        h2![text("Example")],
        code_block_ignore("rust", r#"use yew::prelude::*;
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
        p![
            text("Example: "),
            link!("https://github.com/yewstack/yew/tree/master/examples/simple_ssr", text("simple_ssr")),
        ],
        p![
            text("Example: "),
            link!("https://github.com/yewstack/yew/tree/master/examples/ssr_router", text("ssr_router")),
        ],
        admonition![
            AdmonitionType::Caution,
            None,
            p![
                text("Server-side rendering is currently experimental. If you find a bug, please file \
                  an issue on "),
                link!("https://github.com/yewstack/yew/issues/new?assignees=&labels=bug&template=bug_report.md&title=", text("GitHub")),
                text("."),
            ],
        ],
    ])
);
