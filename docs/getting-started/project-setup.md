---
title: Project Setup
sidebar_label: Introduction
description: Set yourself up for success
---

## Rust

First, you'll need Rust. To install Rust and the `cargo` build tool, follow the [official instructions](https://www.rust-lang.org/tools/install).

You'll also want to install the `wasm32-unknown-unknown` target so you can compile Rust to Wasm.
If you're using rustup, you just need to run `rustup target add wasm32-unknown-unknown`.

## **Wasm Build Tools**

Extra tooling is needed to facilitate the interop between WebAssembly and JavaScript. Additionally,
depending on the tool you choose, they can help make deployment and packaging much less of a
headache by generating all of the JavaScript code necessary to load and run your app's `.wasm`
binary in a browser.

### [**`trunk`**](https://github.com/thedodd/trunk/)

A tool practically made for building Yew apps.
It can build any `wasm-bindgen` based app and its design is inspired by rollup.js.
With Trunk you don't need to have Node.js installed or touch any JavaScript code.
It can automatically bundle assets with your app and even ships with a Sass compiler.

All of our examples are built with Trunk.

[Getting started with `trunk`](project-setup/using-trunk.md)

### [**`wasm-pack`**](https://rustwasm.github.io/docs/wasm-pack/)

A CLI tool developed by the Rust / Wasm Working Group for packaging up WebAssembly. Best used
together with the [`wasm-pack-plugin`](https://github.com/wasm-tool/wasm-pack-plugin) for Webpack.
The primary purpose of `wasm-pack` is building Wasm libraries for use in JavaScript.
Because of this, it can only build libraries and doesn't provide useful tools like a development server or automatic rebuilds.

[Get started with `wasm-pack`](project-setup/using-wasm-pack.md)

### [**`cargo-web`**](https://github.com/koute/cargo-web)

This was the best preferred tool to use before the creation of `wasm-bindgen`.

[Getting started with `cargo web`](project-setup/using-cargo-web.md)

### Comparison

<table>
  <thead>
    <tr>
      <th style="text-align:left"></th>
      <th style="text-align:left"><code>trunk</code></th>
      <th style="text-align:left"><code>wasm-pack</code></th>
      <th style="text-align:left"><code>cargo-web</code></th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td style="text-align:left">Project Status</td>
      <td style="text-align:left">Actively maintained</td>
      <td style="text-align:left">Actively maintained by the <a href="https://rustwasm.github.io/">Rust / Wasm Working Group</a></td>
      <td style="text-align:left">No Github activity for over 6 months</td>
    </tr>
    <tr>
      <td style="text-align:left">Dev Experience</td>
      <td style="text-align:left">Just works! Batteries included, no external dependencies needed.</td>
      <td style="text-align:left">Barebones. You'll need to write some scripts to streamline the experience or use the webpack plugin.</td>
      <td style="text-align:left">Works great for code but needs separate asset pipeline.</td>
    </tr>
    <tr>
      <td style="text-align:left">Local Server</td>
      <td style="text-align:left">Supported</td>
      <td style="text-align:left">Only with webpack plugin</td>
      <td style="text-align:left">Supported</td>
    </tr>
    <tr>
      <td style="text-align:left">Auto rebuild on local changes</td>
      <td style="text-align:left">Supported</td>
      <td style="text-align:left">Only with webpack plugin</td>
      <td style="text-align:left">Supported</td>
    </tr>
    <tr>
      <td style="text-align:left">Asset handling</td>
      <td style="text-align:left">Supported</td>
      <td style="text-align:left">Only with webpack plugin</td>
      <td style="text-align:left">Static assets only</td>
    </tr>
    <tr>
      <td style="text-align:left">Headless Browser Testing</td>
      <td style="text-align:left"><a href="https://github.com/thedodd/trunk/issues/20">In Progress</a></td>
      <td style="text-align:left"><a href="https://rustwasm.github.io/wasm-pack/book/commands/test.html">Supported</a></td>
      <td style="text-align:left"><a href="https://github.com/koute/cargo-web#features">Supported</a></td>
    </tr>
    <tr>
      <td style="text-align:left">Supported Targets</td>
      <td style="text-align:left">
        <ul>
          <li><code>wasm32-unknown-unknown</code></li>
        </ul>
      </td>
      <td style="text-align:left">
        <ul>
          <li><code>wasm32-unknown-unknown</code></li>
        </ul>
      </td>
      <td style="text-align:left">
        <ul>
          <li><code>wasm32-unknown-unknown</code></li>
          <li><code>wasm32-unknown-emscripten</code></li>
          <li><code>asmjs-unknown-emscripten</code></li>
        </ul>
      </td>
    </tr>
    <tr>
      <td style="text-align:left"><code>web-sys</code></td>
      <td style="text-align:left">Compatible</td>
      <td style="text-align:left">Compatible</td>
      <td style="text-align:left">Incompatible</td>
    </tr>
    <tr>
      <td style="text-align:left"><code>stdweb</code></td>
      <td style="text-align:left">Incompatible</td>
      <td style="text-align:left">Compatible</td>
      <td style="text-align:left">Compatible</td>
    </tr>
    <tr>
      <td style="text-align:left">Example Usage</td>
      <td style="text-align:left">
        <a href="build-a-sample-app">Sample app</a>
      </td>
      <td style="text-align:left">
        <a href="https://github.com/yewstack/yew-wasm-pack-minimal">Starter template</a>
      </td>
      <td style="text-align:left">
        <a href="https://www.github.com/yewstack/yew/tree/master/yew-stdweb/examples">Build script</a> for Yew examples
      </td>
    </tr>
  </tbody>
</table>
