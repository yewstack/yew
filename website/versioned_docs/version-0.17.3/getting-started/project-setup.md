---
title: Project Setup
sidebar_label: Introduction
description: Set yourself up for success
id: version-0.17.3-project-setup
original_id: project-setup
---

## Rust

First, you'll need Rust. To install Rust and the `cargo` build tool, follow the [official instructions](https://www.rust-lang.org/tools/install).

## **Wasm Build Tools**

Extra tooling is needed to facilitate the interop between WebAssembly and JavaScript. Additionally, depending on the tool you choose, they can help make deployment and packaging much less of a headache by generating all of the wrapper JavaScript code necessary to run the `.wasm` file from your app in the browser.

### [**`wasm-pack`**](https://rustwasm.github.io/docs/wasm-pack/)

A CLI tool developed by the Rust / Wasm Working Group for packaging up WebAssembly. Best used together with the [`wasm-pack-plugin`](https://github.com/wasm-tool/wasm-pack-plugin) for Webpack.

[Get started with `wasm-pack`](project-setup/using-wasm-pack.md)

### [**`wasm-bindgen`**](https://rustwasm.github.io/docs/wasm-bindgen/)

Both a library and CLI tool and is also developed by the Rust / Wasm Working Group. It is a low level tool \(used internally by `wasm-pack`\) which facilitates JS / WebAssembly interoperability. We don't recommend using `wasm-bindgen`directly because it requires hand-writing some JavaScript to bootstrap your WebAssembly binary. However, it is possible and more info can be found on the [**`wasm-bindgen` guide**](https://rustwasm.github.io/docs/wasm-bindgen/).

[Get started with `wasm-bindgen`](project-setup/using-wasm-bindgen.md)

### [**`cargo-web`**](https://github.com/koute/cargo-web)

The preferred web workflow tool before the introduction of `wasm-pack` and `wasm-bindgen`. It is still the **quickest** way to get up and running and worth installing to run examples that haven't been migrated to support `wasm-pack` yet.

[Getting started with `cargo web`](project-setup/using-cargo-web.md)

### Comparison

<table>
  <thead>
    <tr>
      <th style="text-align:left"></th>
      <th style="text-align:left"><code>wasm-pack</code>
      </th>
      <th style="text-align:left"><code>wasm-bindgen</code>
      </th>
      <th style="text-align:left"><code>cargo-web</code>
      </th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td style="text-align:left">Project Status</td>
      <td style="text-align:left">Actively maintained by the <a href="https://rustwasm.github.io/">Rust / Wasm Working Group</a>
      </td>
      <td style="text-align:left">Actively maintained by the <a href="https://rustwasm.github.io/">Rust / Wasm Working Group</a>
      </td>
      <td style="text-align:left">No Github activity for over 6 months</td>
    </tr>
    <tr>
      <td style="text-align:left">Dev Experience</td>
      <td style="text-align:left">Almost there! Requires <code>webpack</code> for best experience.</td>
      <td
      style="text-align:left">Barebones. You'll need to write some scripts to streamline your dev
        experience.</td>
        <td style="text-align:left">Just works! Batteries included, no external dependencies needed.</td>
    </tr>
    <tr>
      <td style="text-align:left">Local Server</td>
      <td style="text-align:left">Supported with <code>webpack</code> plugin</td>
      <td style="text-align:left">Not supported</td>
      <td style="text-align:left">Supported</td>
    </tr>
    <tr>
      <td style="text-align:left">Auto rebuild on local changes</td>
      <td style="text-align:left">Supported with <code>webpack</code> plugin</td>
      <td style="text-align:left">Not Supported</td>
      <td style="text-align:left">Supported</td>
    </tr>
    <tr>
      <td style="text-align:left">Headless Browser Testing</td>
      <td style="text-align:left"><a href="https://rustwasm.github.io/docs/wasm-pack/commands/test.html">Supported</a>
      </td>
      <td style="text-align:left"><a href="https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html">Supported</a>
      </td>
      <td style="text-align:left"><a href="https://github.com/koute/cargo-web#features">Supported</a>
      </td>
    </tr>
    <tr>
      <td style="text-align:left">Supported Targets</td>
      <td style="text-align:left">
        <ul>
          <li><code>wasm32-unknown-unknown</code>
          </li>
        </ul>
      </td>
      <td style="text-align:left">
        <ul>
          <li><code>wasm32-unknown-unknown</code>
          </li>
        </ul>
      </td>
      <td style="text-align:left">
        <ul>
          <li><code>wasm32-unknown-unknown</code>
          </li>
          <li><code>wasm32-unknown-emscripten</code>
          </li>
          <li><code>asmjs-unknown-emscripten</code>
          </li>
        </ul>
      </td>
    </tr>
    <tr>
      <td style="text-align:left"><code>web-sys</code>
      </td>
      <td style="text-align:left">Compatible</td>
      <td style="text-align:left">Compatible</td>
      <td style="text-align:left">Incompatible</td>
    </tr>
    <tr>
      <td style="text-align:left"><code>stdweb</code>
      </td>
      <td style="text-align:left">Compatible</td>
      <td style="text-align:left">Compatible</td>
      <td style="text-align:left">Compatible</td>
    </tr>
    <tr>
      <td style="text-align:left">Example Usage</td>
      <td style="text-align:left"><a href="https://github.com/yewstack/yew-wasm-pack-minimal">Starter template</a>
      </td>
      <td style="text-align:left"><a href="https://github.com/yewstack/yew/blob/master/examples/build.sh">Build script</a> for
        Yew examples</td>
      <td style="text-align:left"><a href="https://www.github.com/yewstack/yew/tree/master/packages/yew-stdweb/examples">Build script</a> for
        Yew examples</td>
    </tr>
  </tbody>
</table>

