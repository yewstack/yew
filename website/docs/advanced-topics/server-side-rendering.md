---
title: "Server-side Rendering"
description: "Render Yew on the server-side."
---

# Server-side Rendering

By default, Yew applications renders at the client side. That is, a skeleton
html file without any actual content and a WebAssembly bundle is
downloaded to the browser and everything is rendered at the client side.
This is known as client-side rendering.

This approach works fine for most websites as most users now use a modern
browser and devices with adequate computing power.

However, there're some caveats with client-side rendering:

1. The user will not be able to see anything until the entire application is
  downloaded and initial render has completed.
2. Some search engines do not support dynamically rendered web content and
  those who do usually rank dynamic websites lower in the search results.

To solve these problems, we can render our website on the server side.

## How it Works

Yew provides a `YewServerRenderer` renderer to render pages on the
server-side.

You can create a render with `YewServerRenderer::<App>::new()`.
And calling `renderer.render().await` will render `<App />`
into a `String`.

```rust
use yew::prelude::*;
use yew::YewServerRenderer;

#[function_component]
fn App() -> Html {
    html! {<div>{"Hello, World!"}</div>}
}

#[tokio::main]
async fn main() {
    let renderer = YewServerRenderer::<App>::new();

    let rendered = renderer.render().await;

    // Prints: <div>Hello, World!</div>
    println!("{}", rendered);
}
```

## Component Lifecycle

The recommended way of working with server-side rendering is
function components.

All hooks other than `use_effect` will function normally until a component
successfully renders into `Html` for the first time.

Web APIs such as `web_sys` are not available when using server-side rendering.
You application will panic if you try to use them.
You should isolate logics that need Web APIs in `use_effect` or
`use_effect_with_deps` as effects are not executed during server side
rendering.

:::caution

Whilst it's possible to use Struct Components with server-side rendering,
There's no clear boundaries between client-side safe logic like the
`use_effect` hook for struct components.
Struct Components will continue to accept messages until all of its
children is rendered and `destroy` method is called. Developers need to
make sure no messages possibly passed to components would link to logic
that makes use of Web APIs.

:::

## Data Fetching during Server-side Rendering

Data fetching is one of the difficult point with server side rendering
and hydration. Traditionally, when the application renders, it is
instantly available. So there's no mechanism for Yew to detect whether
the application is still fetching. Hence, data client is responsible to implement
a custom solution to detect what's being requested during initial
rendering and triggers a second render after requests are fulfilled.
During the hydration process, the data clients also need to provide a way
to make the data fetched on the server-side available during hydration.

Yew takes a different approach by trying to solve this issue with `<Suspense />`.

Suspense is a special component that when used on the client-side,
provides a way to show a fallback UI while the component is fetching
data (suspended) and resumes to normal UI when data fetching completes.

When the application is rendered on the server-side, Yew waits until a
component is no longer suspended before serializing it to the string
buffer.

Example: [simple\_ssr](https://github.com/yewstack/yew/tree/master/examples/suspense)

:::caution

Server-side rendering is experiemental and currently has no hydration support.
However, you can still use it to generate static websites.

:::
