# Yew Examples

### Running the examples

To start an example, enter its directory and start it with [cargo-web]:

```bash
cargo web start
```

To run an optimised build instead of a debug build use:

```bash
cargo web start --release
```

The `wasm32-unknown-unknown` target will be used by default, which is Rust's native WebAssembly target. The Emscripten-based `wasm32-unknown-emscripten` and `asmjs-unknown-emscripten` targets are also supported if you tell the `cargo-web` to build for them using the `--target` parameter.

[cargo-web]: https://github.com/koute/cargo-web
