### multi_thread

You should compile a worker which have to be spawned in a separate thread:

```sh
wasm-pack build --target no-modules --release -- --features web_sys --bin native_worker
```
