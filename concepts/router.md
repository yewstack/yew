---
description: Yew-Router crate
---

# Router

### Route

Contains a String representing everything after the domain in the url and optionally the state stored in the history api.

### RouteService

Communicates with the browser to get and set Routes

### RouteAgent

Owns a RouteService and is used to coordinate updates when the route changes, either from within the application logic or from an event fired from the browser.

### Switch

The `Switch` trait is used to convert a `Route` to and from the implementer of this trait.

### Router

The Router component communicates with `RouteAgent` and will automatically resolve Routes it gets from the agent into switches, which it will expose via a `render` prop that allows specifying how the resulting switch gets converted to Html&lt;\_&gt;

How to use the router.

