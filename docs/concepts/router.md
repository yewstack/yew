---
title: Router
description: Yew's official router
---

[The router on crates.io](https://crates.io/crates/yew-router)

Routers in Single Page Applications (SPA) handle displaying different pages depending on what the URL is. 
Instead of the default behavior of requesting a different remote resource when a link is clicked, 
the router instead sets the URL locally to point to a valid route in your application. 
The router then detects this change and then decides what to render.

## Core Elements

### `Router`

The Router component. It takes in a callback and renders the HTML based on the returned value of the callback. It is usually placed 
at the top of application. 

## Usage

Routes are defined by an `enum` which derives `Routable`:
```rust
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

```rust
#[function_component(Main)]
fn app() -> Html {
    html! {
        <Router<Route> render=Router::render(switch) />
    }
}

fn switch(routes: Routes) -> Html {
    match route {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Secure => {
            let callback = Callback::from(|_| yew_router::service::push(Routes::Home));
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

To navigate between pages, use either a `Link` component (which renders a `<a>` element) or the `yew_router::service::push` function.

## Relevant examples
- [Router](https://github.com/yewstack/yew/tree/master/examples/router)
