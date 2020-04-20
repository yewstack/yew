### multi_thread

First, build your web app

```sh
cargo build --target wasm32-unknown-unknown --bin multi_thread_app
wasm-bindgen --target web --no-typescript --out-dir static/ --out-name app ../../target/wasm32-unknown-unknown/debug/multi_thread_app.wasm
```

Then, build your web worker

```sh
cargo build --target wasm32-unknown-unknown --bin multi_thread_worker
wasm-bindgen --target no-modules --no-typescript --out-dir static/ --out-name worker ../../target/wasm32-unknown-unknown/debug/multi_thread_worker.wasm
```

Finally, serve the content from the `./static` directory

```sh
python3 -m http.server
```
