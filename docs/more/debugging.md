---
id: debugging
title: Debugging
---

# Debugging

## Panics

Please use the [`console_error_panic`](https://github.com/rustwasm/console_error_panic_hook) crate for nicer stacktraces with Rust symbols. Note, that it is not compatible with apps built with `cargo-web`.

## Console Logging

In general, Wasm web apps are able to interact with Browser APIs, and the `console.log` api is no exception. There are a few options available:

### [`wasm-logger`](https://crates.io/crates/wasm-logger)

This crate integrates with the familiar Rust `log` crate:

```rust
// setup
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
}

// usage
log::info!("Update: {:?}", msg);
```

### [`ConsoleService`](https://docs.rs/yew/latest/yew/services/console/struct.ConsoleService.html)

This service is included within yew and is available when the `"services"` feature is enabled:

```rust
// usage
ConsoleService::new()::info(format!("Update: {:?}", msg));
```

## Source Maps

There is currently no first-class support for source maps for Rust / Wasm web apps. This, of course, is subject to change. If this is no longer true or if progress is made, please suggest a change!

### Latest Info

\[Dec 2019\] [Chrome DevTools update](https://developers.google.com/web/updates/2019/12/webassembly#the_future)

> There is still quite a bit of work to do though. For example, on the tooling side, Emscripten \(Binaryen\) and wasm-pack \(wasm-bindgen\) don’t support updating DWARF information on transformations they perform yet.

\[2020\] [Rust Wasm debugging guide](https://rustwasm.github.io/book/reference/debugging.html#using-a-debugger)

> Unfortunately, the debugging story for WebAssembly is still immature. On most Unix systems, [DWARF](http://dwarfstd.org/) is used to encode the information that a debugger needs to provide source-level inspection of a running program. There is an alternative format that encodes similar information on Windows. Currently, there is no equivalent for WebAssembly.

\[2019\] [Rust Wasm roadmap](https://rustwasm.github.io/rfcs/007-2019-roadmap.html#debugging)

> Debugging is tricky because much of the story is out of this working group's hands, and depends on both the WebAssembly standardization bodies and the folks implementing browser developer tools instead.

