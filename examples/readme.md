# Yew Examples

Use `build.sh` script to build examples. 

Examples are prepared for `wasm-bindgen` except folders which ends with `_wp` which are prepared for `wasm-pack`.

- examples does not use external bundlers and use same `static/index.html` to load application.
- wasm-bindgen builds project using binary crate - `main.rs`
- wasm-pack build project using library only - `lib.rs`

For your own project, you may wish to check out the Yew [starter templates](https://yew.rs/docs/getting-started/starter-templates)


## How to run

```
git clone https://github.com/yewstack/yew.git
cd yew
examples/build.sh minimal # example subfolder
python3 -m http.server --directory examples/static # open localhost:8000 in browser
```


Note: [Visual Studio Code IDE](https://code.visualstudio.com/) has extension [Live Server](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer) which can be used to run example in browser with automatic reload. After install open context menu on `index.html` -> `Open with Live Server`

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
