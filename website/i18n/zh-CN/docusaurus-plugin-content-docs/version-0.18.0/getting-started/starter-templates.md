---
title: 新手模板
---

## `trunk`

- [最简模板](https://github.com/yewstack/yew-trunk-minimal-template) - 通过Trunk构建的小型应用程序，可以帮助快速您入门。

## `wasm-pack`

- [Minimal Template](https://github.com/yewstack/yew-wasm-pack-minimal) - 使用 `wasm-pack` 和 `rollup` 来构建你的应用, 并使用你自己的服务器来部署它，No bells or whistles here.

- [Webpack模板](https://github.com/yewstack/yew-wasm-pack-template) - 使用`wasm-pack`和Webpack 插件 [`wasm-pack-plugin`](https://github.com/wasm-tool/wasm-pack-plugin)来简化开发。

与其他工具不同， `wasm-pack`强制您使用`lib`而不是`bin` crate，并且程序的入口需要用`#[wasm_bindgen(start)]`属性进行标注。

你的 `Cargo.toml` 同样应该指明你的工程的 crate 类型是 "cdylib" 。

```toml
[package]
name = "yew-app"
version = "0.1.0"
authors = ["Yew App Developer <name@example.com>"]
edition = "2018"

[lib]
# You should include "rlib" (the default crate type) otherwise your crate can't be used as a Rust library
# which, among other things, breaks unit testing
crate-type = ["rlib", "cdylib"]

[dependencies]
# for web_sys
yew = "0.17"
# or for stdweb
# yew = { version = "0.17", package = "yew-stdweb" }
wasm-bindgen = "0.2"
```

## 其他模板

- [Parcel Template](https://github.com/spielrs/yew-parcel-template) - 由一位社区成员建立并使用了 [Parcel](https://parceljs.org/) 。
