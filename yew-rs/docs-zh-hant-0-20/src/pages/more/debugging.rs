crate::doc_page!(
    "除錯",
    "/zh-Hant/docs/more/debugging",
    Content::new(vec![
        h2![text("Panics")],
        p![
            text("請使用 "),
            link!(
                "https://github.com/rustwasm/console_error_panic_hook",
                text("console_error_panic"),
            ),
            text(" crate ，他會用 Rust symbols 來做 stacktraces。注意，他跟 "),
            code("cargo-web"),
            text(" 不相容。"),
        ],
        h2![text("Console Logging")],
        p![
            text("通常，Wasm 的網頁應用程式可以跟瀏覽器的 API 互操作，所以 "),
            code("console.log"),
            text(" 這個 api 也不例外，你可以使用以下幾種方法："),
        ],
        h3![text("wasm-logger")],
        p![
            text("這個 crate 整合了令人熟悉的 Rust "),
            code("log"),
            text(" crate："),
        ],
        code_block(
            "rust",
            r#"use log::info;
use wasm_bindgen::JsValue;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let object = JsValue::from("world");
    info!("Hello {}", object.as_string().unwrap());
}"#,
        ),
        h3![text("ConsoleService")],
        p![
            text("Yew 包含了這個 service，而且如果 "),
            code("\"services\""),
            text(" 這個 feature 有被打開的話，你可以直接使用他："),
        ],
        code_block(
            "rust",
            r#"use gloo_console::log;
use wasm_bindgen::JsValue;

fn main() {
    let object = JsValue::from("world");
    log!("Hello", object)
}"#,
        ),
        h2![text("Source Maps")],
        p![text(
            "目前 Rust/Wasm 網頁應用程式，不對 source maps \
             第一線支援。當然，這件事在未來可能會改變，如果這裡寫的資訊不正確，或是事情有所變化，\
             請建議我們修改這篇文件！",
        )],
        h3![text("最新資訊")],
        p![
            text("[2019 12 月] "),
            link!(
                "https://developers.google.com/web/updates/2019/12/webassembly#the_future",
                text("Chrome DevTools update"),
            ),
        ],
        blockquote![p![text(
            "但還是有大量的工作要做。舉例還說，在工具方面，Emscripten (Binaryen) 與 wasm-pack \
             (wasm-bindgen)，還不支援更新轉換他們的行為的 DWARF 資訊。",
        )]],
        p![
            text("[2020] "),
            link!(
                "https://rustwasm.github.io/book/reference/debugging.html#using-a-debugger",
                text("Rust Wasm 除錯指南"),
            ),
        ],
        blockquote![p![
            text("不幸地，WebAssembly 的除錯還不夠完善。在大部分的 Unix 系統中，",),
            link!("http://dwarfstd.org/", text("DWARF")),
            text(
                " 被用來編碼除錯器需要提供的程式碼等級的資訊。還有一種在 Windows \
                 上的編碼資訊。但現在還沒有跟 WebAssembly 等價。",
            ),
        ]],
        p![
            text("[2019] "),
            link!(
                "https://rustwasm.github.io/rfcs/007-2019-roadmap.html#debugging",
                text("Rust Wasm roadmap"),
            ),
        ],
        blockquote![p![text(
            "除錯是一件棘手的事情，因為大部分的事情都不是掌握在這個工作群組中，而是依賴 \
             WebAssembly 的標準，與瀏覽器的開發者工具如何實作。",
        )]],
    ])
);
