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

Routes are defined by an `enum` which derives `Routable`. This enum must be `PartialEq + Clone + Default + Sized + 'static`.

```rust
#[derive(PartialEq, Clone, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[at("/404")]
    NotFound,
}

impl Default for Route {
    fn default() -> Self {
        Self::NotFound
    }
}
```

The `Router` component takes the `Routable` enum as its type parameter, finds the first variant whose path matches the
browser's current URL and passes it to the `render` callback. The callback then decides what to render.
In case no path is matched, the router navigates to a path determined by the `Default` implementation.

`yew_router::current_route` is used to programmatically obtain the current route.
`yew_router::attach_route_listener` is used to attach a listener which is called every time route is changed.

```rust
#[function_component(Main)]
fn app() -> Html {
    html! {
        <Router<Route> render=Router::render(switch) />
    }
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Secure => {
            let callback = Callback::from(|_| yew_router::push_route(Routes::Home));
            html! {
                <div>
                    <h1>{ "Secure" }</h1>
                    <button onclick=callback>{ "Go Home" }</button>
                </div>
            }
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
```

### Navigation

To navigate between pages, use either a `Link` component (which renders a `<a>` element) or the `yew_router::push_route` function.

### Bindings

When a route is matched, you have the option to extract additional data from the URL, which we refer to as "binding".
There are two kinds of binding: implicit and explicit.

#### Implicit binding

If you specify one or more path parameters, these will be implicitly bound to the corresponding fields in your enum variant:

```rust
#[derive(PartialEq, Clone, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/resources/:resource_id")]
    Resource { resource_id: i32 },
}
```

#### Explicit binding

Using the `#[bind(...)]` attribute, you can extract data from elsewhere in the URL. Here is an example
which binds to a query parameter named `p`:

```rust
#[derive(PartialEq, Clone, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/resources")]
    ResourceList {
        #[bind(query_arg = "p")]
        page: i32
    },
}
```

Currently we support the following explicit binds:

- `#[bind(query_arg = "<name>")]`
- `#[bind(query_arg)]`

  Binds to a query parameter with the specified name. If the name is omitted, it defaults to the name
  of the field.

  The type of the field must implement `FromStr + Display`.

- `#[bind(hash_arg = "<name>")]`
- `#[bind(hash_arg)]`

  Binds to a hash parameter with the specified name. If the name is omitted, it defaults to the name
  of the field.

  The type of the field must implement `FromStr + Display`.

- `#[bind(rest)]`

  This binding mode is special as it changes the matching behaviour: instead of attempting to match the
  full URL, only a prefix match is required. The remainder of the URL (including any part of the query
  or hash string that has not already been bound) will be stored in this field.

  The type of the field must itself implement `Routable`.

  This binding mode has two main uses:

  1. Splitting up a complex system of routes into multiple enums. For example, your main `Route`
     enum can have a single variant for `/settings`, and defer further routing to a separate
     `SettingsRoute` enum.

  2. Wildcard matching. Perhaps you don't want to redirect if no route is matched. In that case you
     can make your last variant match the path `/` and bind the rest of the route to a field with
     the built-in `Route` type.

## Relevant examples

- [Router](https://github.com/yewstack/yew/tree/master/examples/router)
