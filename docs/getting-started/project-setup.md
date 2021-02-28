---
title: Project Setup
sidebar_label: Introduction
description: Set yourself up for success
---

## Rust

First, you'll need Rust. To install Rust and the `cargo` build tool, follow the [official instructions](https://www.rust-lang.org/tools/install).

You also need to install the `wasm32-unknown-unknown` target to compile Rust to Wasm.
If you're using rustup, you just need to run `rustup target add wasm32-unknown-unknown`.

:::important
The minimum supported Rust version (MSRV) for Yew is `1.45.0`. Older versions can cause unexpected issues accompanied by incomprehensible error messages.
You can check your toolchain version using `rustup show` (under "active toolchain") or alternatively `rustc --version`. To update your toolchain, run `rustup update`.
:::

## **Wasm Build Tools**

Extra tooling is needed to facilitate the interop between WebAssembly and JavaScript. Additionally,
depending on the tool you choose, they can help make deployment and packaging much less of a
headache by generating all of the JavaScript code necessary to load and run your app's `.wasm`
binary in a browser.

### [**`trunk`**](https://github.com/thedodd/trunk/)

A tool practically made for building Yew apps.
It can build any `wasm-bindgen` based app and its design is inspired by rollup.js.
With Trunk you don't need to have Node.js installed or touch any JavaScript code for that matter.
It can bundle assets for your app and even ships with a Sass compiler.

All of our examples are built with Trunk.

[Getting started with `trunk`](project-setup/using-trunk.md)

### [**`wasm-pack`**](https://rustwasm.github.io/docs/wasm-pack/)

A CLI tool developed by the Rust / Wasm Working Group for packaging up WebAssembly. Best used
together with the [`wasm-pack-plugin`](https://github.com/wasm-tool/wasm-pack-plugin) for Webpack.
The primary purpose of `wasm-pack` is building Wasm libraries for use in JavaScript.
Because of this, it can only build libraries and doesn't provide useful tools like a development server or automatic rebuilds.

[Get started with `wasm-pack`](project-setup/using-wasm-pack.md)

### Comparison

|                               | `trunk`                                                          | `wasm-pack`                                                                                           |
| ----------------------------- | ---------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------  |
| Project Status                | Actively maintained                                              | Actively maintained by the [Rust / Wasm Working Group](https://rustwasm.github.io)                    |
| Dev Experience                | Just works! Batteries included, no external dependencies needed. | Bare-bones. You'll need to write some scripts to streamline the experience or use the webpack plugin. |
| Local Server                  | Supported                                                        | Only with webpack plugin                                                                              |
| Auto rebuild on local changes | Supported                                                        | Only with webpack plugin                                                                              |
| Asset handling                | Supported                                                        | Only with webpack plugin                                                                              |
| Headless Browser Testing      | [In Progress](https://github.com/thedodd/trunk/issues/20)        | [Supported](https://rustwasm.github.io/wasm-pack/book/commands/test.html)                             |
| Supported Targets             | <ul><li><code>wasm32-unknown-unknown</code></li></ul>            | <ul><li><code>wasm32-unknown-unknown</code></li></ul>                                                 |
| Example Usage                 | [Sample app](./build-a-sample-app.md)                            | [Starter template](https://github.com/yewstack/yew-wasm-pack-minimal)                                 |
