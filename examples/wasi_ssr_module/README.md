# WASI SSR Module Example

This example demonstrates how to use the WASI target to run a simple server-side rendering application.

It depends on [wasmtime](https://wasmtime.dev)'s WASI preview2.

> It requires Rust 1.67 or newer.

## Building

To build the example, run the following command from the root of the repository:

```bash
cargo build --manifest-path examples/wasi_ssr_module/Cargo.toml --target wasm32-wasi --release
```

## Running

> Note: This example requires the wasmtime CLI to be installed. See [wasmtime's installation instructions](https://docs.wasmtime.dev/cli-install.html) for more information.

```bash
wasmtime --trap-unknown-imports target/wasm32-wasi/release/wasi_ssr_module.wasm
```

> Warn: This example is not yet fully functional. For some unknown reason, this demo only works [outside this project](https://github.com/celestia-island/tairitsu/blob/a724e3f34754fadf279f036e2c473cbf2abf4b8b/packages/proto/src/html/render.rs) because the dependency `web-sys` includes some objects forcible. It would have crashed when running on this project that caused by unknown import `__wbindgen_placeholder__::__wbindgen_xxx` has not been defined in the WASI environment.
