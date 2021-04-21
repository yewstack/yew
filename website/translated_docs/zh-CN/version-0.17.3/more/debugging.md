---
title: 调试
id: version-0.17.3-debugging
original_id: debugging
---

## Panics

请使用 [`console_error_panic`](https://github.com/rustwasm/console_error_panic_hook) crate 以获得带有 Rust 符号的堆栈跟踪。请注意，它与使用 `cargo-web` 构建的应用程序不兼容。

## Console Logging

通常，Wasm Web 应用程序能够与浏览器 API 进行交互， `console.log` api也不例外。以下是一些可用的选项：

### [`wasm-logger`](https://crates.io/crates/wasm-logger)

这个 crate 集成了我们熟悉的Rust `log` crate：

```rust
// 初始化
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
}

// 用法
log::info!("Update: {:?}", msg);
```

### [`ConsoleService`](https://docs.rs/yew/latest/yew/services/console/struct.ConsoleService.html)

该服务包含在 yew 之中，并且在 `"services"` feature 开启后可用：

```rust
// 使用
ConsoleService::new()::info(format!("Update: {:?}", msg));
```

## Source Maps

目前，Rust / Wasm Web 应用程序的 Source Map 还没有一流支持。当然，这是会改变的。如果在这方面取得了进展，请提出更改建议！

### 近期相关资讯

[Dec 2019] [Chrome DevTools更新](https://developers.google.com/web/updates/2019/12/webassembly#the_future)

> There is still quite a bit of work to do though. For example, on the tooling side, Emscripten (Binaryen) and wasm-pack (wasm-bindgen) don’t support updating DWARF information on transformations they perform yet.

[2020] [ Rust Wasm 调试指南](https://rustwasm.github.io/book/reference/debugging.html#using-a-debugger)

> Unfortunately, the debugging story for WebAssembly is still immature. On most Unix systems, [DWARF](http://dwarfstd.org/) is used to encode the information that a debugger needs to provide source-level inspection of a running program. There is an alternative format that encodes similar information on Windows. Currently, there is no equivalent for WebAssembly.

[2019] [ Rust Wasm 路线图 ](https://rustwasm.github.io/rfcs/007-2019-roadmap.html#debugging)

> Debugging is tricky because much of the story is out of this working group's hands, and depends on both the WebAssembly standardization bodies and the folks implementing browser developer tools instead.
