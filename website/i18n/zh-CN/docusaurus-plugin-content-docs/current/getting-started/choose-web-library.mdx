# 选择 web-sys 还是 stdweb

## 简介

Yew 应用程序可以通过 [`web-sys`](https://docs.rs/web-sys) 或者 [`stdweb`](https://docs.rs/stdweb) 来构建。这两个 crates 提供了 Rust 和 Web API 之间的绑定。当把 `yew` 添加到你的 cargo 依赖时，你需要选择它们其中之一：

{% code title="Cargo.toml" %}
```rust
# 选择 `web-sys`
yew = { version = "0.13", features = ["web_sys"] }

# 选择 `stdweb`
yew = { version = "0.13", features = ["std_web"] }
```
{% endcode %}

我们建议选择 `web-sys`，因为它是由 [Rust / Wasm 工作组](https://rustwasm.github.io/) 提供支持。

## 示例用法

```rust
// web-sys
let window: web_sys::Window = web_sys::window().expect("window not available");
window.alert_with_message("hello from wasm!").expect("alert failed");

// stdweb
let window: stdweb::web::Window = stdweb::web::window();
window.alert("hello from wasm!");

// stdweb 搭配 js! 宏
use stdweb::js;
use stdweb::unstable::TryFrom;
use stdweb::web::Window;

let window_val: stdweb::Value = js!{ return window; }; // <- 里面使用 JS 语法
let window = Window::try_from(window_val).expect("conversion to window failed");
window.alert("hello from wasm!");
```

两个 crate 的 API 略有不用，但他们的目标大致相同，功能相似。

## 选择其中之一

当为你的应用程序选择使用 `web-sys` 还是 `stdweb` 时，有几个不同的角度需要考虑。注意，可以在一个应用程序中同时使用两者，但是为了最小化编译的 `.wasm` 二进制体积，最好选择其中之一。

<table>
  <thead>
    <tr>
      <th style={{ textAlign: "left" }}></th>
      <th style={{ textAlign: "left" }}><code>web-sys</code>
      </th>
      <th style={{ textAlign: "left" }}><code>stdweb</code>
      </th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td style={{ textAlign: "left" }}>&#x9879;&#x76EE;&#x72B6;&#x6001;</td>
      <td style={{ textAlign: "left" }}>&#x7531;<a href="https://rustwasm.github.io/">Rust / Wasm &#x5DE5;&#x4F5C;&#x7EC4;</a>&#x79EF;&#x6781;&#x7EF4;&#x62A4;</td>
      <td
      style={{ textAlign: "left" }}>&#x8D85;&#x8FC7;&#x56DB;&#x4E2A;&#x6708;&#x6CA1;&#x6709; Github &#x6D3B;&#x52A8;</td>
    </tr>
    <tr>
      <td style={{ textAlign: "left" }}>Web API &#x8986;&#x76D6;&#x7387;</td>
      <td style={{ textAlign: "left" }}>Rust API &#x662F;&#x4ECE; Web IDL &#x89C4;&#x8303;&#x81EA;&#x52A8;&#x751F;&#x6210;&#xFF0C;&#x56E0;&#x6B64;&#x7406;&#x8BBA;&#x4E0A;&#x6709;
        100% &#x7684;&#x8986;&#x76D6;&#x7387;&#x3002;</td>
      <td style={{ textAlign: "left" }}>&#x6D4F;&#x89C8;&#x5668; API &#x662F;&#x6839;&#x636E;&#x9700;&#x6C42;&#x7531;&#x793E;&#x533A;&#x6DFB;&#x52A0;</td>
    </tr>
    <tr>
      <td style={{ textAlign: "left" }}>Rust API &#x8BBE;&#x8BA1;</td>
      <td style={{ textAlign: "left" }}>&#x91C7;&#x53D6;&#x4FDD;&#x5B88;&#x7684;&#x65B9;&#x6CD5;&#xFF0C;&#x4E3A;&#x5927;&#x591A;&#x6570;
        API &#x8C03;&#x7528;&#x8FD4;&#x56DE; <code>Result</code>
      </td>
      <td style={{ textAlign: "left" }}>&#x901A;&#x5E38;&#x62D2;&#x7EDD;&#x8FD4;&#x56DE; <code>Result</code> &#x800C;&#x66F4;&#x503E;&#x5411;&#x4E8E;&#x4F7F;&#x7528;
        panic&#x3002;&#x4F8B;&#x5982;&#xFF0C;&#x5728; worker &#x4E2D;&#x8C03;&#x7528; <code>stdweb::web::window()</code> &#x5C06;
        panic&#x3002;</td>
    </tr>
    <tr>
      <td style={{ textAlign: "left" }}>&#x652F;&#x6301;&#x7684;&#x6784;&#x5EFA;&#x5DE5;&#x5177;</td>
      <td style={{ textAlign: "left" }}>
        <ul>
          <li><code>wasm-bindgen</code>
          </li>
          <li><code>wasm-pack</code>
          </li>
        </ul>
      </td>
      <td style={{ textAlign: "left" }}>
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
            <li><code>wasm32-unknown-emscripten</code>
            </li>
            <li><code>asmjs-unknown-emscripten</code>
            </li>
          </ul>
        </td>
    </tr>
  </tbody>
</table>有关更多挑选构建工具的信息，请参阅 [Wasm 构建工具](project-setup/#wasm-build-tools) 指南。

