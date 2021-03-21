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

The Router component. It takes in the routes and renders the HTML based on the current route. It is usually placed 
at the top of application.

### `Route`

A component used to specify the route's path and HTML which is rendered when its parent `Router` component decides it is
the best match for the current route.

### `RouteService`

Interfaces with the `Router`. It can be used to navigate to a specific route or obtain the current route. 

## Usage

`Router` component searches its children for `Route` components and to find the one whose path matches 
the browser's current URL. In case no path is matched, the router navigates to the path specified in `not_found_route` 
prop. If no route is specified, nothing is rendered, and a message is logged to console stating that no route was matched.

### Navigation

In order to navigate between pages, either `Link` component (which renders a `<a>` element) or `RouterService::push` 
function is used.

```rust
let onclick_callback = Callback::from(|_| RouterService::push("/"));
html! {
    <Router not_found_route="/404">
        <Route to="/secure">
            <h1>{"Forbidden"}</h1>
            <button onclick=onclick_callback>{"Navigate to /"}</button>
        </Route>
        <Route to="/">
            <h1>{"Home"}</h1>
            <Link route="/secure">{ "Navigate to /secure" }</Link>
        </Route>
        <Route to="/404">
            <h1>{"Page not found"}</h1>
        </Route>
    </Router>
}
```

## Relevant examples
- [Router](https://github.com/yewstack/yew/tree/master/examples/router)
