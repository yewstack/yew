---
title: 项目设置
sidebar_label: 介绍
description: 为成功做好准备
id: version-0.17.3-project-setup
original_id: project-setup
---

## Rust

首先 ，你需要安装 Rust 。如何安装 Rust 和 `cargo` 构建工具，请参考[官方说明](https://www.rust-lang.org/tools/install)。

## **Wasm 构建工具**

需要安装额外的工具以方便 WebAssembly 与 JavaScript 间的相互操作。此外，根据你选择的工具，他们可以生成所有必需的 JavaScript 包装代码来让你的应用程序中的 `.wasm` 文件运行在浏览器中，从而帮助减轻部署和打包的麻烦。

### [**`wasm-pack`**](https://rustwasm.github.io/docs/wasm-pack/)

一个由 Rust / Wasm 工作组开发的用于打包 WebAssembly 的 CLI 工具。与 Webpack 的 [`wasm-pack-plugin`](https://github.com/wasm-tool/wasm-pack-plugin) 插件搭配使用最佳。

[Get started with `wasm-pack`](project-setup/using-wasm-pack.md)

### [**`wasm-bindgen`**](https://rustwasm.github.io/docs/wasm-bindgen/)

同时是一个库和一个 CLI 工具，也是由 Rust / Wasm 工作组开发。它是一个促进 JS 和 WebAssembly 之间互操作性的底层工具（在 `wasm-pack` 内部被用到）。我们不建议直接使用 `wasm-bindgen` 因为它需要手写一些 JavaScript 代码来引导你的 WebAssembly 二进制程序。但是，直接使用它也是可能的并且可以在 <a href="https://rustwasm.github.io/docs/wasm-bindgen/" data-md-type="link"><strong data-md-type="double_emphasis">`wasm-bindgen` 指南</strong></a> 上找到更多信息。

[Get started with `wasm-bindgen`](project-setup/using-wasm-bindgen.md)

### [**`cargo-web`**](https://github.com/koute/cargo-web)

在 `wasm-pack` 和 `wasm-bindgen` 被介绍前的首选 web 工作流工具。它仍然是**最快捷**的启动和运行方式，值得安装以运行尚未迁移到支持 `wasm-pack` 的示例程序。

[开始使用 cargo-web](project-setup/using-cargo-web.md)

### 对比

<table>
  <thead>
    <tr>
      <th style="text-align:left"></th>
      <th style="text-align:left">
<code>wasm-pack</code>
      </th>
      <th style="text-align:left">
<code>wasm-bindgen</code>
      </th>
      <th style="text-align:left">
<code>cargo-web</code>
      </th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td style="text-align:left">项目状态</td>
      <td style="text-align:left">由 <a href="https://rustwasm.github.io/">Rust / Wasm 工作组</a>积极维护</td>
      <td style="text-align:left">由 <a href="https://rustwasm.github.io/">Rust / Wasm 工作组</a>积极维护</td>
      <td style="text-align:left">超过六个月没有 Github 活动</td>
    </tr>
    <tr>
      <td style="text-align:left">开发体验</td>
      <td style="text-align:left">接近完美！需要 <code>webpack</code> 以获得最佳体验。</td>
      <td style="text-align:left">比较基础。你需要编写一些脚本来简化你的开发体验。</td>
        <td style="text-align:left">管用！自带“电池”，不需要外部依赖。</td>
    </tr>
    <tr>
      <td style="text-align:left">本地服务器</td>
      <td style="text-align:left">通过 <code>webpack</code> 插件支持</td>
      <td style="text-align:left">不支持</td>
      <td style="text-align:left">支持</td>
    </tr>
    <tr>
      <td style="text-align:left">根据本地更改自动重新构建</td>
      <td style="text-align:left">通过 <code>webpack</code> 插件支持</td>
      <td style="text-align:left">不支持</td>
      <td style="text-align:left">支持</td>
    </tr>
    <tr>
      <td style="text-align:left">无头浏览器测试</td>
      <td style="text-align:left">
<a href="https://rustwasm.github.io/docs/wasm-pack/commands/test.html">Supported</a>
      </td>
      <td style="text-align:left">
<a href="https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html">Supported</a>
      </td>
      <td style="text-align:left">
<a href="https://github.com/koute/cargo-web#features">Supported</a>
      </td>
    </tr>
    <tr>
      <td style="text-align:left">支持生成的目标代码</td>
      <td style="text-align:left">
        <ul>
          <li>
<code>wasm32-unknown-unknown</code>
          </li>
        </ul>
      </td>
      <td style="text-align:left">
        <ul>
          <li>
<code>wasm32-unknown-unknown</code>
          </li>
        </ul>
      </td>
      <td style="text-align:left">
        <ul>
          <li>
<code>wasm32-unknown-unknown</code>
          </li>
          <li>
<code>wasm32-unknown-emscripten</code>
          </li>
          <li>
<code>asmjs-unknown-emscripten</code>
          </li>
        </ul>
      </td>
    </tr>
    <tr>
      <td style="text-align:left">
<code>web-sys</code>
      </td>
      <td style="text-align:left">兼容</td>
      <td style="text-align:left">兼容</td>
      <td style="text-align:left">不兼容</td>
    </tr>
    <tr>
      <td style="text-align:left">
<code>stdweb</code>
      </td>
      <td style="text-align:left">兼容</td>
      <td style="text-align:left">兼容</td>
      <td style="text-align:left">兼容</td>
    </tr>
    <tr>
      <td style="text-align:left">示例用法</td>
      <td style="text-align:left">
<a href="https://github.com/yewstack/yew-wasm-pack-minimal">Starter template</a>
      </td>
      <td style="text-align:left">Yew 示例程序的<a href="https://github.com/yewstack/yew/blob/master/examples/build_all.sh">构建脚本</a>       </td>
      <td style="text-align:left">Yew 示例程序的<a href="https://github.com/yewstack/yew/blob/master/examples/build_all.sh">构建脚本</a>       </td>
    </tr>
  </tbody>
</table>
