# 選擇 web-sys 或 stdweb

## 簡介

Yew apps 可以用 `web-sys` 或 `stdweb` 編譯。這兩個 creates 都提供了 Rust 與 WebAPIs 的連結。當把 `yew` 加進你的依賴時，請擇其一使用：

{% code title="Cargo.toml" %}
```rust
# 選擇 `web-sys`
yew = { version = "0.13", features = ["web_sys"] }

# 選擇 `stdweb`
yew = { version = "0.13", features = ["std_web"] }
```
{% endcode %}

我們建議使用 `web-sys`，因為他是由 [Rust / Wasm Working Group](https://rustwasm.github.io/) 維護。

## 使用範例

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

兩個 crates 提供的 APIs 雖然不一樣，但是他們的目標與功能大致相同。

## 比較

以下列出幾點，幫助你從不同的角度去考慮要使用 `web-sys` 還是 `stdweb`。請注意，雖然你可以兩個 crates 都使用，但是為了減少編譯成 `.wasm` 的檔案大小，最好還是只選一個使用。

|  | `web-sys` | `stdweb` |
| :--- | :--- | :--- |


| 專案狀態 | 由 [Rust / Wasm Working Group](https://rustwasm.github.io/) 持續維護中 | GitHub repo 已經有四個月沒有動靜了 |
| :--- | :--- | :--- |


| Web API 覆蓋率 | Rust APIs 是由 Web IDL 自動產生，所以應該已經有 100% 的覆蓋率 | 依社群所需加入 Browser APIs  |
| :--- | :--- | :--- |


| Rust API 設計 | 使用保守的方式，大多的 API 呼叫後會返回 `Result` | 通常比起使用 `Result` 更傾向於直接造成 panic。例如，在 worker 中呼叫 `stdweb::web::window()` 的話就會 panic。 |
| :--- | :--- | :--- |


<table>
  <thead>
    <tr>
      <th style="text-align:left">&#x652F;&#x63F4;&#x7684;&#x5EFA;&#x7F6E;&#x5DE5;&#x5177;</th>
      <th style="text-align:left">
        <ul>
          <li><code>wasm-bindgen</code>
          </li>
          <li><code>wasm-pack</code>
          </li>
        </ul>
      </th>
      <th style="text-align:left">
        <ul>
          <li><code>cargo-web</code>
          </li>
          <li><code>wasm-bindgen</code>
          </li>
          <li><code>wasm-pack</code>
          </li>
        </ul>
      </th>
    </tr>
  </thead>
  <tbody></tbody>
</table><table>
  <thead>
    <tr>
      <th style="text-align:left">&#x652F;&#x63F4;&#x7684;&#x76EE;&#x6A19;&#x5E73;&#x53F0;</th>
      <th style="text-align:left">
        <ul>
          <li><code>wasm32-unknown-unknown</code>
          </li>
        </ul>
      </th>
      <th style="text-align:left">
        <ul>
          <li><code>wasm32-unknown-unknown</code>
          </li>
          <li><code>wasm32-unknown-emscripten</code>
          </li>
          <li><code>asmjs-unknown-emscripten</code>
          </li>
        </ul>
      </th>
    </tr>
  </thead>
  <tbody></tbody>
</table>