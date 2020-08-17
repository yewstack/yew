---
description: Set yourself up for success
---

# 專案設定

## Rust

首先，你的電腦裡必須要安裝 Rust。請參考[官網的教學](https://www.rust-lang.org/tools/install)安裝 Rust 與 `cargo` 這個套件管理工具。

## **Wasm 編譯工具**

我們需要額外的工具來增加 WebAssembly 與 JavaScript 的互操作性。此外，根據你選擇的工具，他們可以產生當你的應用程式運行在瀏覽器時， `.wasm` 檔案所需要的 JavaScript 程式碼，減少佈署與打包的麻煩。

### [**`wasm-pack`**](https://rustwasm.github.io/docs/wasm-pack/)

一套 CLI 工具，由 Rust/Wasm Working Group 為了編譯並打包 WebAssembly 所開發的。最好與 Webpack 的 [`wasm-pack-plugin`](https://github.com/wasm-tool/wasm-pack-plugin) 搭配使用。

[開始使用 wasm-pack](project-setup/using-wasm-pack.md)

### [**`wasm-bindgen`**](https://rustwasm.github.io/docs/wasm-bindgen/)

同時是套件，也是 CLI 工具，並由 Rust / Wasm Working Group 開發。他是一套較底層的工具（通常是 `wasm-pack` 內部使用），用以增加 JavaScript 與 WebAssembly 的互操作性。我們不建議直接使用 `wasm-bindgen`，因為你需要多寫一些 JavaScript 的程式碼來引入你的 WebAssembly 二進位檔案。雖然如此，你仍然可以使用 wasm-bindgen，更多資訊請參考 [**`wasm-bindgen` guide**](https://rustwasm.github.io/docs/wasm-bindgen/)**。**

[開始使用 wasm-bindgen](project-setup/using-wasm-bindgen.md)

### [**`cargo-web`**](https://github.com/koute/cargo-web)

在 `wasm-pack` 與 `wasm-bindgen` 出來之前，這是我們的首選工具。在安裝與執行方面，他的速度仍是最快的，我們推薦你安裝他去執行我們的那些還沒有使用 `wasm-pack` 的範例。

[開始使用 cargo-web](project-setup/using-cargo-web.md)

### 比較

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
      <td style="text-align:left">&#x5C08;&#x6848;&#x72C0;&#x614B;</td>
      <td style="text-align:left">&#x6301;&#x7E8C;&#x7531; <a href="https://rustwasm.github.io/">Rust / Wasm Working Group</a> &#x7DAD;&#x8B77;&#x4E2D;</td>
      <td
      style="text-align:left">&#x6301;&#x7E8C;&#x7531; <a href="https://rustwasm.github.io/">Rust / Wasm Working Group</a> &#x7DAD;&#x8B77;&#x4E2D;</td>
        <td
        style="text-align:left">GitHub Repo &#x5DF2;&#x7D93;&#x516D;&#x500B;&#x6708;&#x4EE5;&#x4E0A;&#x6C92;&#x6709;&#x52D5;&#x975C;&#x4E86;</td>
    </tr>
    <tr>
      <td style="text-align:left">&#x958B;&#x767C;&#x9AD4;&#x9A57;</td>
      <td style="text-align:left">&#x9084;&#x53EF;&#x4EE5;&#xFF0C;&#x9700;&#x8981;&#x642D;&#x914D; Webpack</td>
      <td
      style="text-align:left">&#x592A;&#x904E;&#x5E95;&#x5C64;&#xFF0C;&#x4F60;&#x9700;&#x8981;&#x5BEB;&#x4E00;&#x8173;&#x672C;&#x4F86;&#x512A;&#x5316;&#x958B;&#x767C;&#x9AD4;&#x9A57;</td>
        <td
        style="text-align:left">&#x53EF;&#x4EE5;&#x52D5;&#xFF01;&#x53E6;&#x5916;&#x7121;&#x9700;&#x5916;&#x90E8;&#x5957;&#x4EF6;&#x652F;&#x63F4;</td>
    </tr>
    <tr>
      <td style="text-align:left">&#x672C;&#x5730;&#x7AEF;&#x7684;&#x4F3A;&#x670D;&#x5668; (Local Server)</td>
      <td
      style="text-align:left">&#x642D;&#x914D; <code>webpack</code> &#x63D2;&#x4EF6;</td>
        <td style="text-align:left">&#x4E0D;&#x652F;&#x63F4;</td>
        <td style="text-align:left">&#x652F;&#x63F4;</td>
    </tr>
    <tr>
      <td style="text-align:left">&#x7576;&#x6A94;&#x6848;&#x88AB;&#x66F4;&#x52D5;&#xFF0C;&#x81EA;&#x52D5;&#x91CD;&#x7DE8;&#x8B6F;</td>
      <td
      style="text-align:left">&#x642D;&#x914D; <code>webpack</code> &#x63D2;&#x4EF6;</td>
        <td style="text-align:left">&#x4E0D;&#x652F;&#x63F4;</td>
        <td style="text-align:left">&#x652F;&#x63F4;</td>
    </tr>
    <tr>
      <td style="text-align:left">&#x7121;&#x982D;&#x700F;&#x89BD;&#x5668;&#x6E2C;&#x8A66;</td>
      <td style="text-align:left"><a href="https://rustwasm.github.io/docs/wasm-pack/commands/test.html">Supported</a>
      </td>
      <td style="text-align:left"><a href="https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html">Supported</a>
      </td>
      <td style="text-align:left"><a href="https://github.com/koute/cargo-web#features">Supported</a>
      </td>
    </tr>
    <tr>
      <td style="text-align:left">&#x652F;&#x63F4;&#x7684;&#x76EE;&#x6A19;&#x7A0B;&#x5F0F;&#x78BC;</td>
      <td
      style="text-align:left">
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
      <td style="text-align:left">&#x76F8;&#x5BB9;</td>
      <td style="text-align:left">&#x76F8;&#x5BB9;</td>
      <td style="text-align:left">&#x4E0D;&#x76F8;&#x5BB9;</td>
    </tr>
    <tr>
      <td style="text-align:left"><code>stdweb</code>
      </td>
      <td style="text-align:left">&#x76F8;&#x5BB9;</td>
      <td style="text-align:left">&#x76F8;&#x5BB9;</td>
      <td style="text-align:left">&#x76F8;&#x5BB9;</td>
    </tr>
    <tr>
      <td style="text-align:left">&#x7BC4;&#x4F8B;&#x7528;&#x6CD5;</td>
      <td style="text-align:left"><a href="https://github.com/yewstack/yew-wasm-pack-minimal">&#x65B0;&#x624B;&#x6A21;&#x677F;</a>
      </td>
      <td style="text-align:left">Yew &#x7BC4;&#x4F8B;&#x7684;<a href="https://github.com/yewstack/yew/blob/master/examples/build_all.sh">&#x7DE8;&#x8B6F;&#x8173;&#x672C;</a>
      </td>
      <td style="text-align:left">Yew &#x7BC4;&#x4F8B;&#x7684;<a href="https://github.com/yewstack/yew/blob/master/examples/build_all.sh">&#x7DE8;&#x8B6F;&#x8173;&#x672C;</a>
      </td>
    </tr>
  </tbody>
</table>