crate::doc_page!(
    "除錯",
    "/zh-Hant/docs/more/debugging",
    Content::new(vec![
        h2(vec![text("Panics")]),
        p(vec![
            text("請使用 "),
            link(
                "https://github.com/rustwasm/console_error_panic_hook",
                vec![code("console_error_panic")],
            ),
            text(" crate，他會用 Rust symbols 來做 stacktraces。注意，他跟 "),
            code("cargo-web"),
            text(" 不相容。"),
        ]),
        h2(vec![text("Console Logging")]),
        p(vec![text(
            "通常，Wasm 的網頁應用程式可以跟瀏覽器的 API 互操作，所以 console.log 這個 api \
             也不例外，你可以使用以下幾種方法：",
        )]),
        h3(vec![link(
            "https://crates.io/crates/wasm-logger",
            vec![code("wasm-logger")],
        )]),
        p(vec![
            text("這個 crate 整合了令人熟悉的 Rust "),
            code("log"),
            text(" crate："),
        ]),
        code_block(
            "rust",
            r#"// 設定
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
}

// 使用
log::info!("Update: {:?}", msg);"#,
        ),
        h3(vec![link(
            "https://docs.rs/yew/0.13.2/yew/services/console/struct.ConsoleService.html",
            vec![code("ConsoleService")],
        )]),
        p(vec![
            text("Yew 包含了這個 service，而且如果 "),
            code("\"services\""),
            text(" 這個 feature 有被打開的話，你可以直接使用他："),
        ]),
        code_block(
            "rust",
            r#"// 使用
ConsoleService::info(format!("Update: {:?}", msg).as_ref());"#,
        ),
        h2(vec![text("Source Maps")]),
        p(vec![text(
            "目前 Rust/Wasm 網頁應用程式，不對 source maps \
             第一線支援。當然，這件事在未來可能會改變，如果這裡寫的資訊不正確，或是事情有所變化，\
             請建議我們修改這篇文件！",
        )]),
        h3(vec![text("最新資訊")]),
        p(vec![
            text("\\[2019 12 月\\] "),
            link(
                "https://developers.google.com/web/updates/2019/12/webassembly#the_future",
                vec![text("Chrome DevTools update")],
            ),
        ]),
        blockquote(vec![p(vec![text(
            "但還是有大量的工作要做。舉例還說，在工具方面，Emscripten (Binaryen) 與 wasm-pack \
             (wasm-bindgen)，還不支援更新轉換他們的行為的 DWARF 資訊。",
        )])]),
        p(vec![
            text("\\[2020\\] "),
            link(
                "https://rustwasm.github.io/book/reference/debugging.html#using-a-debugger",
                vec![text("Rust Wasm 除錯指南")],
            ),
        ]),
        blockquote(vec![p(vec![
            text("不幸地，WebAssembly 的除錯還不夠完善。在大部分的 Unix 系統中，"),
            link("http://dwarfstd.org/", vec![text("DWARF")]),
            text(
                " 被用來編碼除錯器需要提供的程式碼等級的資訊。還有一種在 Windows \
                 上的編碼資訊。但現在還沒有跟 WebAssembly 等價。",
            ),
        ])]),
        p(vec![
            text("\\[2019\\] "),
            link(
                "https://rustwasm.github.io/rfcs/007-2019-roadmap.html#debugging",
                vec![text("Rust Wasm roadmap")],
            ),
        ]),
        blockquote(vec![p(vec![text(
            "除錯是一件棘手的事情，因為大部分的事情都不是掌握在這個工作群組中，而是依賴 \
             WebAssembly 的標準，與瀏覽器的開發者工具如何實作。",
        )])]),
    ])
);
