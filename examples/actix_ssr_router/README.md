# SSR Router Example

This example is the same as the `axum_ssr_router`, except the
server side is served with `actix_web` instead of axum.

# How to run this example

1. Build Hydration Bundle

`trunk build`

2. Run the server

`cargo run --features=ssr --bin ssr_router_server -- --dir dist`
