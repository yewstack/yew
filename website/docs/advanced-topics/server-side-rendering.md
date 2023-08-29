---
title: 'Server-side Rendering'
description: 'Render Yew on the server-side.'
---

# Server-side Rendering

By default, Yew components render on the client side. When a viewer
visits a website, the server sends a skeleton HTML file without any actual
content and a WebAssembly bundle to the browser.
Everything is rendered on the client side by the WebAssembly
bundle. This is known as client-side rendering.

This approach works fine for most websites, with some caveats:

1. Users will not be able to see anything until the entire WebAssembly
   bundle is downloaded and the initial render has been completed.
   This can result in a poor experience for users on a slow network.
2. Some search engines do not support dynamically rendered web content and
   those who do usually rank dynamic websites lower in the search results.

To solve these problems, we can render our website on the server side.

## How it Works

Yew provides a `ServerRenderer` to render pages on the
server side.

To render Yew components on the server side, you can create a renderer
with `ServerRenderer::<App>::new()` and call `renderer.render().await`
to render `<App />` into a `String`.

```rust
use yew::prelude::*;
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
}
```

## Component Lifecycle

The recommended way of working with server-side rendering is
function components.

All hooks other than `use_effect` (and `use_effect_with`)
will function normally until a component successfully renders into `Html`
for the first time.

:::caution Web APIs are not available!

Web APIs such as `web_sys` are not available when your component is
rendering on the server side.
Your application will panic if you try to use them.
You should isolate logics that need Web APIs in `use_effect` or
`use_effect_with` as effects are not executed during server-side rendering.

:::

:::danger Struct Components

While it is possible to use Struct Components with server-side rendering,
there are no clear boundaries between client-side safe logic like the
`use_effect` hook for function components and lifecycle events are invoked
in a different order than the client side.

In addition, Struct Components will continue to accept messages until all of its
children are rendered and `destroy` method is called. Developers need to
make sure no messages possibly passed to components would link to logic
that makes use of Web APIs.

When designing an application with server-side rendering support,
prefer function components unless you have a good reason not to.

:::

## Data Fetching during Server-side Rendering

Data fetching is one of the difficult points with server-side rendering and hydration.

Traditionally, when a component renders, it is instantly available
(outputs a virtual DOM to be rendered). This works fine when the
component does not want to fetch any data. But what happens if the component
wants to fetch some data during rendering?

In the past, there was no mechanism for Yew to detect whether a component is still
fetching data. The data-fetching client is responsible to implement
a solution to detect what is being requested during the initial render and triggers
a second render after requests are fulfilled. The server repeats this process until
no more pending requests are added during a render before returning a response.

This not only wastes CPU resources by repeatedly rendering components,
but the data client also needs to provide a way to make the data fetched on the
server side available during the hydration process to make sure that the
virtual DOM returned by the initial render is consistent with the
server-side rendered DOM tree which can be hard to implement.

Yew takes a different approach by trying to solve this issue with `<Suspense />`.

Suspense is a special component that when used on the client side, provides a
way to show a fallback UI while the component is fetching
data (suspended) and resumes to normal UI when the data fetching completes.

When the application is rendered on the server side, Yew waits until a
component is no longer suspended before serializing it into the string
buffer.

During the hydration process, elements within a `<Suspense />` component
remains dehydrated until all of its child components are no longer
suspended.

With this approach, developers can build a client-agnostic, SSR-ready
application with data fetching with very little effort.

## SSR Hydration

Hydration is the process that connects a Yew application to the
server-side generated HTML file. By default, `ServerRender` prints
hydratable HTML string which includes additional information to facilitate hydration.
When the `Renderer::hydrate` method is called, instead of starting rendering from
scratch, Yew will reconcile the Virtual DOM generated by the application
with the HTML string generated by the server renderer.

:::caution

To successfully hydrate an HTML representation created by the
`ServerRenderer`, the client must produce a Virtual DOM layout that
exactly matches the one used for SSR including components that do not
contain any elements. If you have any component that is only useful in
one implementation, you may want to use a `PhantomComponent` to fill the
position of the extra component.
:::

:::warning

The hydration can only succeed if the real DOM matches the expected DOM
after initial render of the SSR output (static HTML) by browser. If your HTML is
not spec-compliant, the hydration _may_ fail. Browsers may change the DOM structure
of the incorrect HTML, causing the actual DOM to be different from the expected DOM.
For example, [if you have a `<table>` without a `<tbody>`, the browser may add a `<tbody>` to the DOM](https://github.com/yewstack/yew/issues/2684)
:::

## Component Lifecycle during hydration

During Hydration, components schedule 2 consecutive renders after it is
created. Any effects are called after the second render completes.
It is important to make sure that the render function of your
component is free of side effects. It should not mutate any states or trigger
additional renders. If your component currently mutates states or triggers
additional renders, move them into a `use_effect` hook.

It is possible to use Struct Components with server-side rendering in
hydration, the view function will be called
multiple times before the rendered function will be called.
The DOM is considered as not connected until the rendered function is called,
you should prevent any access to rendered nodes
until `rendered()` method is called.

## Example

```rust ,ignore
use yew::prelude::*;
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
}
```

Example: [simple_ssr](https://github.com/yewstack/yew/tree/master/examples/simple_ssr)
Example: [ssr_router](https://github.com/yewstack/yew/tree/master/examples/ssr_router)

:::caution

Server-side rendering is currently experimental. If you find a bug, please file
an issue on [GitHub](https://github.com/yewstack/yew/issues/new?assignees=&labels=bug&template=bug_report.md&title=).

:::
