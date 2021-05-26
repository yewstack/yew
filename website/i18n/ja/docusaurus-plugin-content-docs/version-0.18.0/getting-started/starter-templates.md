---
title: Starter templates
---

## `wasm-pack`

* [ミニマルテンプレート](https://github.com/yewstack/yew-wasm-pack-minimal) - アプリをビルドするのに `wasm-pack`と

  `rollup`を使い、サーバーはアプリをサーブします. ベルや笛はここにはありません。

* [Webpackテンプレート](https://github.com/yewstack/yew-wasm-pack-template) - `wasm-pack`と

  [`wasm-pack-plugin`](https://github.com/wasm-tool/wasm-pack-plugin)を使い、Webpackが開発を滑らかにします。

これらのテンプレートを使うのと`cargo-web`を使用するのと重要な違いは、このアプローチは`bin`クレートではなく`lib`クレートを用いて
`#[wasm_bindgen]`によってエントリーポイントを指定できる点です。

また、`Cargo.toml`はクレートの種類が"cdylib"であると特定できるようにしましょう。

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

## その他のテンプレート

* [Parcel Template](https://github.com/spielrs/yew-parcel-template) - コミュニティのメンバーによって開発され、
Parcel](https://parceljs.org/)を使っています。

