# Server-side Rendering Example

This example demonstrates server-side rendering.

The hydration bundle must be built first with the following command:

`trunk build examples/simple_ssr/index.html`

Then run `cargo run --bin simple_ssr_server -- --dir examples/simple_ssr/dist` and navigate to http://localhost:8080/ to
view results.
