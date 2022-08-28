# Server-side Rendering Example

This example demonstrates server-side rendering.

# How to run this example

Execute the following commands from the `examples/` folder of the repo

1. Build hydration bundle

`trunk build simple_ssr/index.html`

This builds static artifacts that will be served and places them in `simple_ssr/dist`.

2. Run the server

`cargo run --features=ssr --bin simple_ssr_server -- --dir simple_ssr/dist`

3. Open Browser

Navigate to http://localhost:8080/ to view results.
