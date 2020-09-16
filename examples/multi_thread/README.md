# Multi-Thread Example

First, build your web app

```bash
cargo build --target wasm32-unknown-unknown --bin app
wasm-bindgen --target web --out-dir dist ../../target/wasm32-unknown-unknown/debug/app.wasm
```

Then, build your web worker

```bash
cargo build --target wasm32-unknown-unknown --bin worker
wasm-bindgen --target no-modules --out-dir dist ../../target/wasm32-unknown-unknown/debug/worker.wasm
```
