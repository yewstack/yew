### multi_thread

You should compile a worker which have to be spawned in a separate thread:

```sh
cargo web build --bin native_worker --release --features std_web
```

For `web-sys` support, use `no-modules` output.

```sh
wasm-pack build --target no-modules --release -- --features web_sys --bin native_worker
```
