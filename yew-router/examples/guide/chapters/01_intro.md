# Intro

## What is Yew Router?
Yew Router is a router in the style of [React Router](https://reacttraining.com/react-router/web/guides/quick-start).
A router's job in the context of a frontend web application is to take part of a URL and determine what HTML to render based on that.



## Important Constructs
Yew router contains a service, an agent, routing components, and components for changing the route.
You can choose to forgo using the router itself and just use the service or agent, although the `Router` provides a higher layer of abstraction over the same domain.

#### `route!` macro
The `route!` macro allows you to specify a string that determines how the router will match part of a URL.

#### Routing Components
The `Router` and `Route` components allow you to specify what route strings to match, and what content render when they succeed.
You can tell a `Route` to render a component directly, or you can provide a closure to render an arbitrary piece of html.

#### Accessory Components
The `RouterLink` and `RouterButton` components wrap links and buttons respectively and provide ready-made components that can be used to change the route.

#### Service
The routing service interfaces directly with the browser's history API. 
You can register a callback to receive messages about when routes change, or you can change the route yourself.

#### Agent
The routing agent offers a layer of orchestration to an application. 
It sits between the routing components and the service, and provides an interface for you to change the route and make sure the router itself gets notified of the change.

------
### Example
This crate allows you to specify which components to render as easily as: 
```rust
html! {
    <Router>
        <Route matcher=route!("/a/{}" CaseInsensitive) render=component::<AModel>() />
        <Route matcher=route!("/c") render=component::<CModel>() />
    </Router>
}
```