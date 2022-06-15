# Server-side Rendering Example

This example demonstrates server-side rendering.

# How to run this example

1. build hydration bundle

`trunk build examples/simple_ssr/index.html`

2. Run the server

`cargo run --features=ssr --bin simple_ssr_server -- --dir examples/simple_ssr/dist`

3. Open Browser

Navigate to http://localhost:8080/ to view results.
