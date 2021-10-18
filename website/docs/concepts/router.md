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

The Router component. It takes in a callback and renders the HTML based on the returned value of the callback. It is usually placed
at the top of the application.

Routes are defined by an `enum` which derives `Routable`. This enum must be `Clone + Sized.
```rust
use yew_router::Routable;

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

The `Router` component takes the `Routable` enum as its type parameter, finds the first variant whose path matches the
browser's current URL and passes it to the `render` callback. The callback then decides what to render.
In case no path is matched, the router navigates to the path with `not_found` attribute. If no route is specified,
nothing is rendered, and a message is logged to console stating that no route was matched.

`yew_router::current_route` is used to programmatically obtain the current route.
`yew_router::attach_route_listener` is used to attach a listener which is called every time route is changed.

```rust
use yew_router::{Routable, Switch, BrowserRouter};
use yew_router::hooks::use_history;
use yew_router::history::History;
use yew::{Callback, function_component, html, Html};

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

# #[function_component(Main)]
# fn app() -> Html {
html! {
    <BrowserRouter>
        <Switch<Route> render={Switch::render(switch)} />
    </BrowserRouter>
}
# }

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
```

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
