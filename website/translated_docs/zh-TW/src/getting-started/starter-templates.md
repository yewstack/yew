# 初始模板

## `wasm-pack`

* [Minimal Template](https://github.com/yewstack/yew-wasm-pack-minimal) - 使用 `wasm-pack` 與 `rollup` 來建置應用程式。你必須用你自己的伺服器服務來啟動它。 
* [Webpack Template](https://github.com/yewstack/yew-wasm-pack-template) - 使用 `wasm-pack` 與 [`wasm-pack-plugin`](https://github.com/wasm-tool/wasm-pack-plugin)，用 Wepack 套件來簡化開發流程。 

使用上述的模板與單純使用 `cargo-web` 最重要的區別在於，模板使用的是 `lib` 而不是 `bin` crate，而且程式的進入點會註解 `#[wasm_bindgen]`

你的 `Cargo.toml` 必須要設定一個 `cdylib` 的 crate-type。

{% code title="Cargo.toml" %}
```text
[package]
name = "yew-app"
version = "0.1.0"
authors = ["Yew App Developer <name@example.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
# for web_sys
yew = 0.15
# or for stdweb
# yew = { version = "0.15", package = "yew-stdweb" }
wasm-bindgen = "0.2"
```
{% endcode %}

## 其他的模板

* [Parcel Template](https://github.com/spielrs/yew-parcel-template) - 由一個社群成員用 [Parcel](https://parceljs.org/) 製作

