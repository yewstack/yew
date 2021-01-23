---
title: Starter templates
---

## `trunk`

- [Minimal Template](https://github.com/yewstack/yew-trunk-minimal-template) - A small application built with Trunk to get you started.

## `wasm-pack`

- [Minimal Template](https://github.com/yewstack/yew-wasm-pack-minimal) - Uses `wasm-pack` and
  `rollup` to build your application, and your own server to serve it. No bells or whistles here.

- [Webpack Template](https://github.com/yewstack/yew-wasm-pack-template) - Uses `wasm-pack` and the
  [`wasm-pack-plugin`](https://github.com/wasm-tool/wasm-pack-plugin) for Webpack to streamline development.

Unlike other tools, `wasm-pack` forces you to use a `lib`, not a `bin` crate,
and the entry-point to your program is annotated with a `#[wasm_bindgen(start)]` attribute.

Your `Cargo.toml` also should specify that your crate's type is a "cdylib".

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
yew = "0.17"
wasm-bindgen = "0.2"
```

## Other templates

- [Parcel Template](https://github.com/spielrs/yew-parcel-template) - Created by a community member
  and uses [Parcel](https://parceljs.org/)
