# Server-side Rendering Example

This example demonstrates server-side rendering.

# Running

1. Build hydration bundle

`trunk build index.html`

This builds static artifacts that will be served and places them in `dist`.

2. Run the server

`cargo run --features=ssr --bin simple_ssr_server -- --dir dist`