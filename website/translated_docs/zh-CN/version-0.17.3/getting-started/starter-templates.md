---
title: 新手模板
id: version-0.17.3-starter-templates
original_id: starter-templates
---

## `wasm-pack`

- [Minimal Template](https://github.com/yewstack/yew-wasm-pack-minimal) - 使用 `wasm-pack` 和

     <code>rollup</code> 来构建你的应用, 并使用你自己的服务器来部署它，No bells or whistles here.

- [Webpack Template](https://github.com/yewstack/yew-wasm-pack-template) - 使用 `wasm-pack` 和

    [`wasm-pack-plugin`](https://github.com/wasm-tool/wasm-pack-plugin) 来简化开发。

使用这些例子和使用 `cargo-web` 的最重要的区别是 它们 使用了 `lib` 类型 而非 `bin` 类型的工程，同时你的应用的入口应该使用 `#[wasm_bindgen]` 标记出来。

你的 `Cargo.toml` 同样应该指明你的工程的 crate-type 是 "cdylib" 。

```text
[package]
name = "yew-app"
version = "0.1.0"
authors = ["Yew App Developer <name@example.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
# 使用 web_sys
yew = "0.17"
# 或是使用 stdweb
# yew = { version = "0.17", package = "yew-stdweb" }
wasm-bindgen = "0.2"
```

## 其他模板

- [Parcel Template](https://github.com/spielrs/yew-parcel-template) - 由一位社区成员建立并使用了 [Parcel](https://parceljs.org/) 。
