# Server-side Rendering Example

This example demonstrates server-side rendering.

# Running

1. Build hydration bundle

`trunk build index.html`

1. Run the server

`cargo run --features=ssr --bin simple_ssr_server -- --dir dist`