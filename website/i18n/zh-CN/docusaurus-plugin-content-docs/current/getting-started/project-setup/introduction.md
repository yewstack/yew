---
description: 为成功做好准备
---

# 项目设置

## Rust

首先 ，你需要安装 Rust 。如何安装 Rust 和 `cargo` 构建工具，请参考[官方说明](https://www.rust-lang.org/tools/install)。

## **Wasm 构建工具**

需要安装额外的工具以方便 WebAssembly 与 JavaScript 间的相互操作。此外，根据你选择的工具，他们可以生成所有必需的 JavaScript 包装代码来让你的应用程序中的 `.wasm` 文件运行在浏览器中，从而帮助减轻部署和打包的麻烦。

### [**`wasm-pack`**](https://rustwasm.github.io/docs/wasm-pack/)

一个由 Rust / Wasm 工作组开发的用于打包 WebAssembly 的 CLI 工具。与 Webpack 的 [`wasm-pack-plugin`](https://github.com/wasm-tool/wasm-pack-plugin) 插件搭配使用最佳。

[开始使用 wasm-pack](project-setup/using-wasm-pack.md)

### [**`wasm-bindgen`**](https://rustwasm.github.io/docs/wasm-bindgen/)

同时是一个库和一个 CLI 工具，也是由 Rust / Wasm 工作组开发。它是一个促进 JS 和 WebAssembly 之间互操作性的底层工具（在 `wasm-pack` 内部被用到）。我们不建议直接使用 `wasm-bindgen` 因为它需要手写一些 JavaScript 代码来引导你的 WebAssembly 二进制程序。但是，直接使用它也是可能的并且可以在 [**`wasm-bindgen` 指南**](https://rustwasm.github.io/docs/wasm-bindgen/) 上找到更多信息。

[开始使用 wasm-bindgen](project-setup/using-wasm-bindgen.md)

### [**`cargo-web`**](https://github.com/koute/cargo-web)

在 `wasm-pack` 和 `wasm-bindgen` 被介绍前的首选 web 工作流工具。它仍然是**最快捷**的启动和运行方式，值得安装以运行尚未迁移到支持 `wasm-pack` 的示例程序。

[开始使用 cargo-web](project-setup/using-cargo-web.md)

### 对比

<table>
  <thead>
    <tr>
      <th style={{ textAlign: "left" }}></th>
      <th style={{ textAlign: "left" }}><code>wasm-pack</code>
      </th>
      <th style={{ textAlign: "left" }}><code>wasm-bindgen</code>
      </th>
      <th style={{ textAlign: "left" }}><code>cargo-web</code>
      </th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td style={{ textAlign: "left" }}>&#x9879;&#x76EE;&#x72B6;&#x6001;</td>
      <td style={{ textAlign: "left" }}>&#x7531; <a href="https://rustwasm.github.io/">Rust / Wasm &#x5DE5;&#x4F5C;&#x7EC4;</a>&#x79EF;&#x6781;&#x7EF4;&#x62A4;</td>
      <td
      style={{ textAlign: "left" }}>&#x7531; <a href="https://rustwasm.github.io/">Rust / Wasm &#x5DE5;&#x4F5C;&#x7EC4;</a>&#x79EF;&#x6781;&#x7EF4;&#x62A4;</td>
        <td
        style={{ textAlign: "left" }}>&#x8D85;&#x8FC7;&#x516D;&#x4E2A;&#x6708;&#x6CA1;&#x6709; Github &#x6D3B;&#x52A8;</td>
    </tr>
    <tr>
      <td style={{ textAlign: "left" }}>&#x5F00;&#x53D1;&#x4F53;&#x9A8C;</td>
      <td style={{ textAlign: "left" }}>&#x63A5;&#x8FD1;&#x5B8C;&#x7F8E;&#xFF01;&#x9700;&#x8981; <code>webpack</code> &#x4EE5;&#x83B7;&#x5F97;&#x6700;&#x4F73;&#x4F53;&#x9A8C;&#x3002;</td>
      <td
      style={{ textAlign: "left" }}>&#x6BD4;&#x8F83;&#x57FA;&#x7840;&#x3002;&#x4F60;&#x9700;&#x8981;&#x7F16;&#x5199;&#x4E00;&#x4E9B;&#x811A;&#x672C;&#x6765;&#x7B80;&#x5316;&#x4F60;&#x7684;&#x5F00;&#x53D1;&#x4F53;&#x9A8C;&#x3002;</td>
        <td
        style={{ textAlign: "left" }}>&#x7BA1;&#x7528;&#xFF01;&#x81EA;&#x5E26;&#x201C;&#x7535;&#x6C60;&#x201D;&#xFF0C;&#x4E0D;&#x9700;&#x8981;&#x5916;&#x90E8;&#x4F9D;&#x8D56;&#x3002;</td>
    </tr>
    <tr>
      <td style={{ textAlign: "left" }}>&#x672C;&#x5730;&#x670D;&#x52A1;&#x5668;</td>
      <td style={{ textAlign: "left" }}>&#x901A;&#x8FC7; <code>webpack</code> &#x63D2;&#x4EF6;&#x652F;&#x6301;</td>
      <td
      style={{ textAlign: "left" }}>&#x4E0D;&#x652F;&#x6301;</td>
        <td style={{ textAlign: "left" }}>&#x652F;&#x6301;</td>
    </tr>
    <tr>
      <td style={{ textAlign: "left" }}>&#x6839;&#x636E;&#x672C;&#x5730;&#x66F4;&#x6539;&#x81EA;&#x52A8;&#x91CD;&#x65B0;&#x6784;&#x5EFA;</td>
      <td
      style={{ textAlign: "left" }}>&#x901A;&#x8FC7; <code>webpack</code> &#x63D2;&#x4EF6;&#x652F;&#x6301;</td>
        <td
        style={{ textAlign: "left" }}>&#x4E0D;&#x652F;&#x6301;</td>
          <td style={{ textAlign: "left" }}>&#x652F;&#x6301;</td>
    </tr>
    <tr>
      <td style={{ textAlign: "left" }}>&#x65E0;&#x5934;&#x6D4F;&#x89C8;&#x5668;&#x6D4B;&#x8BD5;</td>
      <td style={{ textAlign: "left" }}><a href="https://rustwasm.github.io/docs/wasm-pack/commands/test.html">&#x652F;&#x6301;</a>
      </td>
      <td style={{ textAlign: "left" }}><a href="https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html">&#x652F;&#x6301;</a>
      </td>
      <td style={{ textAlign: "left" }}><a href="https://github.com/koute/cargo-web#features">&#x652F;&#x6301;</a>
      </td>
    </tr>
    <tr>
      <td style={{ textAlign: "left" }}>&#x652F;&#x6301;&#x751F;&#x6210;&#x7684;&#x76EE;&#x6807;&#x4EE3;&#x7801;</td>
      <td
      style={{ textAlign: "left" }}>
        <ul>
          <li><code>wasm32-unknown-unknown</code>
          </li>
        </ul>
        </td>
        <td style={{ textAlign: "left" }}>
          <ul>
            <li><code>wasm32-unknown-unknown</code>
            </li>
          </ul>
        </td>
        <td style={{ textAlign: "left" }}>
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
      <td style={{ textAlign: "left" }}><code>web-sys</code>
      </td>
      <td style={{ textAlign: "left" }}>&#x517C;&#x5BB9;</td>
      <td style={{ textAlign: "left" }}>&#x517C;&#x5BB9;</td>
      <td style={{ textAlign: "left" }}>&#x4E0D;&#x517C;&#x5BB9;</td>
    </tr>
    <tr>
      <td style={{ textAlign: "left" }}><code>stdweb</code>
      </td>
      <td style={{ textAlign: "left" }}>&#x517C;&#x5BB9;</td>
      <td style={{ textAlign: "left" }}>&#x517C;&#x5BB9;</td>
      <td style={{ textAlign: "left" }}>&#x517C;&#x5BB9;</td>
    </tr>
    <tr>
      <td style={{ textAlign: "left" }}>&#x793A;&#x4F8B;&#x7528;&#x6CD5;</td>
      <td style={{ textAlign: "left" }}><a href="https://github.com/yewstack/yew-wasm-pack-minimal">&#x65B0;&#x624B;&#x6A21;&#x677F;</a>
      </td>
      <td style={{ textAlign: "left" }}>Yew &#x793A;&#x4F8B;&#x7A0B;&#x5E8F;&#x7684;<a href="https://github.com/yewstack/yew/blob/master/examples/build_all.sh">&#x6784;&#x5EFA;&#x811A;&#x672C;</a>
      </td>
      <td style={{ textAlign: "left" }}>Yew &#x793A;&#x4F8B;&#x7A0B;&#x5E8F;&#x7684;<a href="https://github.com/yewstack/yew/blob/master/examples/build_all.sh">&#x6784;&#x5EFA;&#x811A;&#x672C;</a>
      </td>
    </tr>
  </tbody>
</table>
