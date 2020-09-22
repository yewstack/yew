---
title: Choosing a web library
---

## Introduction

Yew apps can be built using either [`web-sys`](https://docs.rs/web-sys) or [`stdweb`](https://docs.rs/stdweb).
These two crates provide the bindings between Rust and Web APIs. You'll need to choose one or the other when adding
`yew` to your cargo dependencies:

```toml
# Choose `web-sys`
yew = "0.17"

# Choose `stdweb`
yew = { version = "0.17", package = "yew-stdweb" }
```

We recommend using `web-sys` due to its support from the [Rust / Wasm Working Group](https://rustwasm.github.io/).

:::warning
Yew will freeze support for `stdweb` at v0.18.
It will still receive patch fixes, but no new features will be added.
See [#1569](https://github.com/yewstack/yew/issues/1569)
:::

## Example Usage

This example illustrates the difference in how the two libraries are used.
You don't need to run this yourself.

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

The APIs for the two crates differ slightly but they serve roughly the same purpose.

## Choosing One

There are a few different angles to consider when choosing between using `web-sys` and `stdweb` for your app.
Note that it's possible to use both in one app, but to minimize the binary size of your compiled crate it's best to use only one of the two.

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
      <td style="text-align:left">Project Status</td>
      <td style="text-align:left">Actively maintained by the <a href="https://rustwasm.github.io/">Rust / Wasm Working Group</a>
      </td>
      <td style="text-align:left">No Github activity for over 8 months</td>
    </tr>
    <tr>
      <td style="text-align:left">Web API Coverage</td>
      <td style="text-align:left">Rust APIs are generated from the Web IDL spec</td>
      <td style="text-align:left">Browser APIs are added as needed by the community</td>
    </tr>
    <tr>
      <td style="text-align:left">Rust API Design</td>
      <td style="text-align:left">Takes conservative approach by returning <code>Result</code> for most API
        calls</td>
      <td style="text-align:left">Often avoids <code>Result</code> in favor of panics. For instance, <code>stdweb::web::window()</code> will
        panic when called in a worker</td>
    </tr>
    <tr>
      <td style="text-align:left">Supported Build Tools</td>
      <td style="text-align:left">
        <p></p>
        <ul>
          <li><code>trunk</code>
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
        </ul>
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
          <li><code>wasm32-unknown-emscripten</code>
          </li>
          <li><code>asmjs-unknown-emscripten</code>
          </li>
        </ul>
      </td>
    </tr>
  </tbody>
</table>
