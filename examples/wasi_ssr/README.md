# WASI SSR Example

This example demonstrates how to use the WASI target to run a simple server-side rendering application.

## Building

To build the example, run the following command from the root of the repository:

```bash
cargo build --package wasi_ssr --target wasm32-wasi
```

> TODO - It needs to write a `bin` target for this example that it's only runnable on `wasmtime_wasi::preview2`.

## Running

```bash
cargo run --package wasi_ssr --bin wasi_ssr
```
