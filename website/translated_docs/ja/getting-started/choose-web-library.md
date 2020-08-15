---
id: web-library
title: Choosing a web library
---
## はじめに

Yewアプリは[`web-sys`](https://docs.rs/web-sys)か[`stdweb`](https://docs.rs/stdweb)で作ることができます。
これらのクレートはRustとWeb APIのバインディングを提供しています。
Cargoの依存クレートに`yew`を追加する際はどちらかを選ばなければいけません:

```toml
# Choose `web-sys`
yew = "0.17"

# Choose `stdweb`
yew = { version = "0.17", package = "yew-stdweb" }
```

[Rust / Wasm 活動チーム](https://rustwasm.github.io/)のサポートがある`web-sys`が推奨です。

## 使用例

```rust
// web-sys
let window: web_sys::Window = web_sys::window().expect("window not available");
window.alert_with_message("hello from wasm!").expect("alert failed");

// stdweb
let window: stdweb::web::Window = stdweb::web::window();
window.alert("hello from wasm!");

// stdweb with js! macro
use stdweb::js;
use stdweb::unstable::TryFrom;
use stdweb::web::Window;

let window_val: stdweb::Value = js!{ return window; }; // <- JS syntax inside!
let window = Window::try_from(window_val).expect("conversion to window failed");
window.alert("hello from wasm!");
```

2つのクレートのAPIはわずかに異なりますが、だいたい同じ目的で似た機能が提供されています。

## 一方を選ぶ

アプリに`web-sys`と`stdweb`のどちらを選ぶかにおいてはいくつかの見方があります。
注意として、一つのアプリに両方を用いることができるのですが、クレートをコンパイルした際にバイナリのサイズを小さくするには
一方だけを使用するのが良いです。

<table>
  <thead>
    <tr>
      <th style="text-align:left"></th>
      <th style="text-align:left"><code>web-sys</code>
      </th>
      <th style="text-align:left"><code>stdweb</code>
      </th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td style="text-align:left">プロジェクトの進捗状況</td>
      <td style="text-align:left">
        <a href="https://rustwasm.github.io/">Rust / Wasm 活動チーム</a>により活発にメンテナンスされている
      </td>
      <td style="text-align:left">GitHubで8ヶ月間動きなし</td>
    </tr>
    <tr>
      <td style="text-align:left">Web API のカバー</td>
      <td style="text-align:left">RustのAPIはWeb IDLスペックから自動的に生成される。</td>
      <td style="text-align:left">Browser APIはコミュニティにより追加。</td>
    </tr>
    <tr>
      <td style="text-align:left">RustのAPIのデザイン</td>
      <td style="text-align:left">
        ほとんどのAPIコールおいて<code>Result</code>が返ってくるよう保守的なアプローチがとられている。
      </td>
      <td style="text-align:left">しばしば<code>Result</code>を返さずpanicするようになっている。例えば <code>stdweb::web::window()</code>ワーカーの中で呼ばれるパニックする。</td>
    </tr>
    <tr>
      <td style="text-align:left">サポートされているビルドツール</td>
      <td style="text-align:left">
        <p></p>
        <ul>
          <li><code>wasm-bindgen</code>
          </li>
          <li><code>wasm-pack</code>
          </li>
        </ul>
      </td>
      <td style="text-align:left">
        <p></p>
        <ul>
          <li><code>cargo-web</code>
          </li>
          <li><code>wasm-bindgen</code>
          </li>
          <li><code>wasm-pack</code>
          </li>
        </ul>
      </td>
    </tr>
    <tr>
      <td style="text-align:left">サポートされているターゲット</td>
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
  </tbody>
</table>

