---
title: 除錯
id: 版本-0.17.3-除錯
original_id: 除錯
---

## Panics

請使用 [`console_error_panic`](https://github.com/rustwasm/console_error_panic_hook) crate ，他會用 Rust symbols 來做 stacktraces。注意，他跟 `cargo-web` 不相容。

## Console Logging

通常，Wasm 的網頁應用程式可以跟瀏覽器的 API 互操作，所以 `console.log` 這個 api 也不例外，你可以使用以下幾種方法：

### [`wasm-logger`](https://crates.io/crates/wasm-logger)

這個 crate 整合了令人熟悉的 Rust `log` crate：

```rust
// 設定
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
}

// 使用
log::info!("Update: {:?}", msg);
```

### [`ConsoleService`](https://docs.rs/yew/latest/yew/services/console/struct.ConsoleService.html)

Yew 包含了這個 service，而且如果 `"services"` 這個 feaure 有被打開的話，你可以直接使用他：

```rust
// 使用
ConsoleService::new()::info(format!("Update: {:?}", msg));
```

## Source Maps

目前 Rust/Wasm 網頁應用程式，不對 source maps 第一線支援。當然，這件事在未來可能會改變，如果這裡寫的資訊不正確，或是事情有所變化，請建議我們修改這篇文件！

### 最新資訊

[2019 12月] [Chrome DevTools update](https://developers.google.com/web/updates/2019/12/webassembly#the_future)

> 但還是有大量的工作要做。舉例還說，在工具方面，Emscripten (Binaryen) 與 wasm-pack (wasm-bindgen)，還不支援更新轉換他們的行為的 DWARF 資訊。

[2020] [Rust Wasm 除錯指南](https://rustwasm.github.io/book/reference/debugging.html#using-a-debugger)

> 不幸地，WebAssembly 的除錯還不夠完善。在大部分的 Unix 系統中，[DWARF](http://dwarfstd.org/) 被用來編碼除錯器需要提供的程式碼等級的資訊。還有一種在 Windows 上的編碼資訊。但現在還沒有跟 WebAssembly 等價。

[2019] [Rust Wasm roadmap](https://rustwasm.github.io/rfcs/007-2019-roadmap.html#debugging)

> 除錯是一件棘手的事情，因為大部分的事情都不是掌握在這個工作群組中，而是依賴 WebAssembly 的標準，與瀏覽器的開發者工具如何實作。
