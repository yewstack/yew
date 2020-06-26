# Choosing between web-sys and stdweb

## Introduction

Yew apps can be built with either [`web-sys`](https://docs.rs/web-sys) or 
[`stdweb`](https://docs.rs/stdweb). These two crates provide the bindings between Rust and Web APIs. 
You'll need to choose one or the other when adding `yew` to your cargo dependencies:

{% code title="Cargo.toml" %}
```rust
# Choose `web-sys`
yew = "0.16"

# Choose `stdweb`
yew = { version = "0.16", package = "yew-stdweb" }
```
{% endcode %}

We recommend using `web-sys` due to its support from the [Rust / Wasm Working Group](https://rustwasm.github.io/).

## Example Usage

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

The APIs for the two crates differ slightly but they serve roughly the same purpose with similar 
functionality.

## Choosing One

There are a few different angles to consider when choosing between using `web-sys` and `stdweb` for 
your app. Note that it's possible to use both in one app, but to minimize the binary size of your 
compiled crate it's best to use only one of the two.

|  | `web-sys` | `stdweb` |
| :--- | :--- | :--- |


| Project Status | Actively maintained by the [Rust / Wasm Working Group](https://rustwasm.github.io/) | No Github activity for over 4 months |
| :--- | :--- | :--- |


| Web API Coverage | Rust APIs are auto-generated from the Web IDL spec and so should have 100% coverage. | Browser APIs are added as needed by the community |
| :--- | :--- | :--- |


| Rust API Design | Takes conservative approach by returning `Result` for most API calls | Often avoids `Result` in favor of panics. For instance, `stdweb::web::window()` will panic when called in a worker. |
| :--- | :--- | :--- |


<table>
  <thead>
    <tr>
      <th style="text-align:left">Supported Build Tools</th>
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
      <th style="text-align:left">Supported Targets</th>
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
