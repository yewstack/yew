# Suspense SSR Example

SSR with a `<Suspense>` boundary around a slow async data fetch
(`use_prepared_state!`). The fallback streams to the browser immediately;
resolved content swaps in when the fetch completes.

See the [Server-Side Rendering docs](https://yew.rs/docs/advanced-topics/server-side-rendering#out-of-order-streaming) for details.

See also: [simple_ssr](../simple_ssr) for a basic SSR example.

# Running

1. Build hydration bundle

`trunk build`

2. Run the server

`cargo run --features=ssr --bin suspense_ssr_server -- --dir dist`
