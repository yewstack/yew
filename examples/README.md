# Yew Examples

In order to build the examples, use the `build.sh` script.

All the examples are designed to work with `wasm-bindgen` except for examples in folders ending in `_wp` (these are prepared for `wasm-pack`).

- The examples do not use external bundlers and all use the same `static/index.html` after being built.
- `wasm-bindgen` builds projects as binary crates (`main.rs`)
- `wasm-pack` builds projects as library crates (`lib.rs`)

Have a look at Yew's [starter templates](https://yew.rs/docs/getting-started/starter-templates) when starting a project using Yew – they can significantly simplify things.


## How to run the examples

```sh
git clone https://github.com/yewstack/yew.git
cd yew/examples
git checkout v0.17 # switch to current version branch
./run_example.sh minimal # builds and opens the "minimal" example in browser
```


Note: [Visual Studio Code](https://code.visualstudio.com/) has an extension called [Live Server](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer) which can be used to run examples in the browser (with automatic page refreshes when a file is changed). After installing the extension open `index.html` and press `Open with Live Server` in the context menu.

## Requirements

The default way to build the examples is by using `wasm-bindgen` (this is automatically installed if you've installed `wasm-pack`). If they aren't installed, these tools can be installed by using `cargo` (`cargo install wasm-pack wasm-bindgen-cli`).

Installation guides: [Rust](https://www.rust-lang.org/learn/get-started) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```bash
# rust install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# wasm-pack install
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh;
# wasm-bindgen-cli install
cargo install wasm-bindgen-cli
```
