---
title: "Router"
description: "Yew's official router"
---

[The router on crates.io](https://crates.io/crates/yew-router)

Routers in Single Page Applications (SPA) handle displaying different pages depending on what the URL is.
Instead of the default behavior of requesting a different remote resource when a link is clicked,
the router instead sets the URL locally to point to a valid route in your application.
The router then detects this change and then decides what to render.

## Usage

You start by defining a `Route`.

Routes are defined as an `enum` which derives `Routable`. This enum must be `Clone + PartialEq`.

```rust
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}
```

A `Route` is paired with a `<Switch />` component, which finds the first variant whose path matches the
browser's current URL and passes it to the `render` callback. The callback then decides what to render.
In case no path is matched, the router navigates to the path with `not_found` attribute. If no route is specified,
nothing is rendered, and a message is logged to console stating that no route was matched.

```rust
use yew_router::prelude::*;;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let history = use_history().unwrap();

    let onclick_callback = Callback::from(move |_| history.push(Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button onclick={onclick_callback}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <Switch<Route> render={Switch::render(switch)} />
    }
}
```

Finally, you need to register the `<Router />` component as a context.
`<Router />` provides session history information to its children.

When using `yew-router` in browser environment, `<BrowserRouter />` is
recommended.

```rust
use yew_router::prelude::*;;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let history = use_history().unwrap();

    let onclick_callback = Callback::from(move |_| history.push(Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button onclick={onclick_callback}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
```

### Path Segments

It is also possible to extract information from a route.

```rust
# use yew_router::prelude::*;
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/post/:id")]
    Post { id: String },
    // ...
}
```

You can then access the post's id inside `<Switch />` and forward it to the appropriate component via properties.

```rust ,ignore
fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Post { id } => <Post {id} />,
        // ...
    }
}
```

Linking to a specific post is as easy as passing the variant to `Link`:

```rust ,ignore
<Link<Route> to={Route::Post { id: "new-yew-release".to_string() }}>{ "Yew v0.19 out now!" }</Link</Route>>
```

For more information about the route syntax and how to bind parameters, check out [route-recognizer](https://docs.rs/route-recognizer/0.3.1/route_recognizer/#routing-params).

### History and Location

The router provides a universal `History` and `Location` struct which
can be used to access routing information. They can be retrieved by
hooks or convenient functions on `ctx.link()`.

They have a couple flavours:

#### `AnyHistory` and `AnyLocation`

These types are available with all routers and should be used whenever possible.
They implement a subset of `window.history` and `window.location`.

You can access them using the following hooks:

- `use_history`
- `use_location`

#### `BrowserHistory` and `BrowserLocation`

These are only available when `<BrowserRouter />` is used. They provide
additional functionality that is not available in `AnyHistory` and
`AnyLocation` (such as: `location.host`).

### Navigation

To navigate between pages, use either a `Link` component (which renders a `<a>` element), the `history.push` function, or the `history.replace` function, which replaces the current page in the user's browser history instead of pushing a new one onto the stack.

### Listening to Changes

#### Functional components

Simply use available hooks `use_history`, `use_location` and `use_route`.
Your components will re-render when provided values change.

#### Struct components

In order to react on route changes, you can pass a callback closure to the `listen()` method of `AnyHistory`.

:::note
The history listener will get unregistered once it is dropped. Make sure to store the handle inside your component state.
:::

```rust ,ignore
fn create(ctx: &Context<Self>) -> Self {
    let _listener = ctx.link()
        .history()
        .unwrap()
        .listen(ctx.link().callback(
            // handle event
        ));
    MyComponent {
        _listener
    }
}
```

### Query Parameters

#### Specifying query parameters when navigating

In order to specify query parameters when navigating to a new route, use either `history.push_with_query` or the `history.replace_with_query` functions.
It uses `serde` to serialize the parameters into query string for the URL so any type that implements `Serialize` can be passed.
In its simplest form this is just a `HashMap` containing string pairs.

#### Obtaining query parameters for current route

`location.query` is used to obtain the query parameters.
It uses `serde` to deserialize the parameters from query string in the URL.

## Relevant examples

- [Router](https://github.com/yewstack/yew/tree/master/examples/router)
