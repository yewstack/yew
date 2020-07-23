# Using wasm-bindgen

## Install

```bash
cargo install wasm-bindgen-cli
```

## Build

First build the app which will generate a wasm file. Suppose you want to build the app from
[build a sample app](../build-a-sample-app.md). The path of the generated file will be
`target/wasm32-unknown-unknown/debug/yew-app.wasm`.

```bash
cargo build --target wasm32-unknown-unknown
```

Then, run the wasm-bindgen CLI. This command will produce the files in the `--out-dir`
directory with your app's compiled WebAssembly along with a JavaScript wrapper which can be 
used to start your application. For the [build a sample app](../build-a-sample-app.md), we
want the files to be generated in the `static` folder (`--out-dir` static ), and should be 
named wasm.js and wasm_bg.wasm ( `--out-name` wasm).

```bash
wasm-bindgen --target web --out-dir static --out-name wasm target/wasm32-unknown-unknown/debug/appname.wasm --no-typescript
```

## Serve

Feel free to use your preferred server. Here we use a simple python server to serve the app

```bash
python -m http.server 8000
```

## Supported Targets

* `wasm32-unknown-unknown`

