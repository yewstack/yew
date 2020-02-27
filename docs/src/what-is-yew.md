---
description: A high level overview of the framework.
---

# What Is Yew?

## What is Yew?

Yew is a frontend web framework, similar to React or Elm, that allows you to build websites with complex logic that runs in a web browser. Apps that use Yew are written in Rust, which compiles to Web Assembly \(**WASM**\), or plain JavaScript, in order to run in the browser.

### Value Proposition

Rust is the best positioned language that compiles to WASM given its nearly non-existent runtime \(leading to smaller file sizes\), and its various language features that allow safe usage of WASM as a target, while achieving maximum performance. Yew is a Rust/WASM framework, the architecture of which should be familiar to anyone who has used React or Elm, that allows you to build fast, small, and correct frontend web applications.

### What Makes Up Yew?

Yew is comprised of a few distinct parts that are used to create a working application.

* `html!` macro - A procedural macro that creates a tree that represents HTML that will be shown in the browser.
* `Component` trait - Specifies how a data structure in Rust can be displayed in, as well as interact with, the browser.
* `Properties` trait - Allows components to pass state to child components.
* `Callback` event system - Allows child components, actors, or HTML elements to send messages to components.
* `Agent` trait - Specifies actors that can coordinate global state, or run independent tasks on web workers.
* `Services` - Rust glue code to APIs present in the browser. Examples include: fetch requests, timers, console access, and more.

#### Dependencies

Yew is built on top of `StdWeb`, a library that provides bindings between Rust and the Browser. Some features rely on another library called `web-sys`, which is auto-generated from web browser specification documents, and makes use of `wasm_bindgen`.

#### Build environments

If your app is architected to only use StdWeb-based features, you can use the `cargo-web` build tool to build, test, and run your application. If you want to make use of advanced features, or just prefer the ecosystem, you can use various existing JS bundlers and their `wasm_bindgen` based plugins to build your app. These include building using `wasm-pack` and bundling it yourself using `rollup`, or using `Webpack` or `Parcel` to manage your development and deployment tasks.

`cargo-web` supports compiling to JS via `Emscripten` or compiling to WASM using `rustc`, while using `wasm_bindgen` based approaches only support compiling to WASM.
