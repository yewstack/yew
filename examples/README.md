# Yew Examples

Use `build.sh` script to build examples.

Examples are prepared for `wasm-bindgen` except folders ending with `_wp` which are prepared for `wasm-pack`.

- The examples do not use external bundlers and all use the same `static/index.html` after being built.
- `wasm-bindgen` builds projects as binary crates (`main.rs`)
- `wasm-pack` builds projects as library crates (`lib.rs`)

For your own project, you may wish to check out the Yew [starter templates](https://yew.rs/docs/getting-started/starter-templates)


## How to run

```sh
git clone https://github.com/yewstack/yew.git
cd yew/examples
./build.sh minimal # example subfolder
python3 -m http.server --directory static # open localhost:8000 in browser
```


Note: [Visual Studio Code IDE](https://code.visualstudio.com/) has the [Live Server](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer) extension which can be used to run examples in the browser with automatic reload. After installing, open the context menu on `index.html` -> `Open with Live Server`

## Requirements

Default way to build is using `wasm-bindgen` which comes with `wasm-pack`

- Install using cargo: `cargo install wasm-pack`

Install guides: [Rust](https://www.rust-lang.org/learn/get-started) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```bash
# rust install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# wasm-pack install
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh;
```
