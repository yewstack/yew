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

### `RouteService`

Interfaces with the `Router`. It can be used to navigate to a specific route. 

## Usage

Routes are defined by an `enum` which derives `Routable`:
```rust
enum Routes {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}
```

`Router` component takes that enum that defines the routes as its type parameter, finds the one whose path matches the 
browser's current URL and passes it to the `render` callback. The callback returns the HTML which is to be rendered. 
In case no path is matched, the router navigates to the path with `not_found` attribute. If no route is specified, 
nothing is rendered, and a message is logged to console stating that no route was matched.

```rust
#[function_component(Main)]
fn app() -> Html {
    html! {
        <Router<Routes> render=Router::render(switch) />
    }
}

fn switch(routes: Routes) -> Html {
    let onclick_callback = Callback::from(|_| RouterService::push(Routes::Home, None));
    match routes {
        Routes::Home => html! { <h1>{ "Home" }</h1> },
        Routes::Secure => html! {
            <div>
                <h1>{ "Secure" }</h1>
                <button onclick=onclick_callback>{ "Go Home" }</button>
            </div>
        },
        Routes::NotFound => html! {<h1>{"404"}</h1>},
    }
}
```

### Navigation

In order to navigate between pages, either `Link` component (which renders a `<a>` element) or `RouterService::push` 
function is used.

## Relevant examples
- [Router](https://github.com/yewstack/yew/tree/master/examples/router)
