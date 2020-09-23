---
title: Using wasm-pack
---

This tool was created by the Rust / Wasm Working Group for building WebAssembly applications. It supports packaging code into `npm` modules and has an accompanying [Webpack plugin](https://github.com/wasm-tool/wasm-pack-plugin) for easy integration with existing JavaScript applications. More information is given in [the `wasm-pack` documentation](https://rustwasm.github.io/docs/wasm-pack/introduction.html).

:::note
`wasm-pack` requires that you set the crate-type explicitly to include `cdylib`:

```toml
[lib]
crate-type = ["rlib", "cdylib"]
```

:::

## Install

```bash
cargo install wasm-pack
```

## Build

This command will produce a bundle in the `./pkg` directory with your app's compiled WebAssembly
along with a JavaScript wrapper which can be used to start your application.

```bash
wasm-pack build --target web
```

## Bundle

For more information on rollup.js visit this [guide](https://rollupjs.org/guide/en/#quick-start).

```bash
rollup ./main.js --format iife --file ./pkg/bundle.js
```

When using a bundler like rollup.js you can omit `--target web`.

## Serve

Feel free to use your preferred server. Here we use a simple Python server to serve the built app.

```bash
python -m http.server 8000
```

If you don't have Python installed, you can install and use the [`simple-http-server`](https://github.com/TheWaWaR/simple-http-server) crate instead.
