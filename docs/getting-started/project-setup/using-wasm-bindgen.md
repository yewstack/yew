---
id: wasm-bindgen
title: Using wasm-bindgen
---

## Install

```bash
cargo install wasm-bindgen-cli
```

## Build

First build the app which will generate a Wasm file. Suppose you want to build the app from
[build a sample app](../build-a-sample-app.md). The path of the outputted file would be
`target/wasm32-unknown-unknown/debug/yew-app.wasm`. If you've named your crate
something different, the name of the Wasm file won't be `yew-app.wasm` and will instead
be whatever you've set `package.name` to in your `Cargo.toml` file.

```bash
cargo build --target wasm32-unknown-unknown
```

Then, run wasm-bindgen's CLI. This command will produce a set of files in the `--out-dir` directory
containing both your app's compiled WebAssembly and a JavaScript wrapper which will load 
the Wasm binary and run it. This is necessary because browsers currently can't load WebAssembly files
directly instead requiring them to be loaded via Javascript scripts. In the [build a sample app](../build-a-sample-app.md) example we want the files to be generated in the `static` folder (to do this you'll need
to pass `--out-dir static` as  a flag to `wasm-bindgen`) and be called `wasm.js` and `wasm_bg.wasm` 
(you can do this by passing `--out-name wasm` as a flag to `wasm-bindgen`).

```bash
wasm-bindgen --target web --out-dir static --out-name wasm target/wasm32-unknown-unknown/debug/appname.wasm --no-typescript
```

## Serving your application

Feel free to use your preferred server. Here we use a simple python server to serve the app

```bash
python -m http.server 8000
```

## Supported targets

* `wasm32-unknown-unknown`
## Further reading
* [The `wasm-bindgen` docs](https://rustwasm.github.io/docs/wasm-bindgen/)
