---
id: starter-templates
title: Starter templates
---

## `wasm-pack`

* [Minimal Template](https://github.com/yewstack/yew-wasm-pack-minimal) - Uses `wasm-pack` and 

  `rollup` to build your application, and your own server to serve it. No bells or whistles here.

* [Webpack Template](https://github.com/yewstack/yew-wasm-pack-template) - Uses `wasm-pack` and the 

  [`wasm-pack-plugin`](https://github.com/wasm-tool/wasm-pack-plugin) for Webpack to streamline development.

The important distinction between these templates and using `cargo-web` is that this approach uses a `lib`, not a `bin` crate, and the entry-point to your program is annotated with a `#[wasm_bindgen]` annotation.

Your `Cargo.toml` also should specify that your crate's type is a "cdylib".

```text title="Cargo.toml"
[package]
name = "yew-app"
version = "0.1.0"
authors = ["Yew App Developer <name@example.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
# for web_sys
yew = "0.17"
# or for stdweb
# yew = { version = "0.17", package = "yew-stdweb" }
wasm-bindgen = "0.2"
```

## Other templates

* [Parcel Template](https://github.com/spielrs/yew-parcel-template) - Created by a community member 
and uses [Parcel](https://parceljs.org/)

