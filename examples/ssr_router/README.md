# SSR Router Example

This example is the same as the function router example, but with
server-side rendering and hydration support. It reuses the same codebase
of the function router example.

# How to run this example

1. Build Hydration Bundle

`trunk build index.html`

2. Run the server

`cargo run --features=ssr --bin ssr_router_server -- --dir dist`
