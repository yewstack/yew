# WASI SSR Module Example

This example demonstrates how to use the WASI target to run a simple server-side rendering application.

It depends on [wasmtime](https://wasmtime.dev)'s WASI preview2.

## Building

To build the example, run the following command from the root of the repository:

```bash
cargo build --manifest-path examples/wasi_ssr_module/Cargo.toml --target wasm32-wasip1 --release
```

## Running

> Note: This example requires the wasmtime CLI to be installed. See [wasmtime's installation instructions](https://docs.wasmtime.dev/cli-install.html) for more information.

```bash
wasmtime target/wasm32-wasip1/release/wasi_ssr_module.wasm
```

> Note: If your wasmtime CLI throws an error that it says some imports like `__wbindgen_placeholder__::__wbindgen_xxx` is invalid, try to run `cargo update`. See issue [rustwasm/gloo#411](https://github.com/rustwasm/gloo/pull/411#discussion_r1421219033).
