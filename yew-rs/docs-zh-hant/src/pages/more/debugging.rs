pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["意外終止 (Panics)"],
        p!["Yew 會自動在瀏覽器控制台中輸出意外終止日誌。"],
        h2!["控制台日誌"],
        p![
            "在 JavaScript 中，",
            code("console.log()"),
            " 用於輸出到瀏覽器控制台。以下是一些 Yew 的選項。",
        ],
        h3![link!("https://crates.io/crates/wasm-logger", code("wasm-logger"))],
        p![
            code("wasm-logger"),
            " crate 與 ",
            link!("https://crates.io/crates/log", code("log")),
            " crate 集成，以將日誌等級、來源行和檔案名稱傳送到瀏覽器控制台。",
        ],
        code_block("rust", r#"use log::info;
use wasm_bindgen::JsValue;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let object = JsValue::from("world");
    info!("Hello {}", object.as_string().unwrap());
}"#),
        h3![link!("https://crates.io/crates/gloo-console", code("gloo-console"))],
        p![
            "這個 crate 是 Gloo 的一部分，提供了對瀏覽器 API 的 Rust 包裝。 ",
            code("log!"),
            " 巨集可以直接接受 ",
            code("JsValue"),
            "，比 ",
            code("wasm_logger"),
            " 更容易使用。",
        ],
        code_block("rust", r#"use gloo_console::log;
use wasm_bindgen::JsValue;

fn main() {
    let object = JsValue::from("world");
    log!("Hello", object)
}"#),
        h3![link!("https://crates.io/crates/tracing-web", code("tracing-web"))],
        p![
            code("tracing-web"),
            " 可以與 ",
            link!("https://crates.io/crates/tracing-subscriber", code("tracing-subscriber")),
            " 一起使用，將訊息輸出到瀏覽器控制台。",
        ],
        code_block("rust", r#"use tracing_subscriber::{
    fmt::{
        format::{FmtSpan, Pretty},
        time::UtcTime,
    },
    prelude::*,
};
use wasm_bindgen::JsValue;

fn main() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_timer(UtcTime::rfc_3339())
        .with_writer(tracing_web::MakeConsoleWriter)
        .with_span_events(FmtSpan::ACTIVE);
    let perf_layer = tracing_web::performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();
    let object = JsValue::from("world");
    tracing::info!("Hello {}", object.as_string().unwrap());
}"#),
        h2!["偵錯元件生命週期"],
        p![
            link!("https://crates.io/crates/tracing", code("tracing")),
            " 可用於收集與組件生命週期相關的事件資訊。 ",
            code("tracing"),
            " 還附帶一個 ",
            code("log"),
            " 支援的特性標誌，可以與 ",
            code("wasm-logger"),
            " 很好地整合。",
        ],
        p![
            link!("https://docs.rs/tracing/latest/tracing/level_filters/index.html#compile-time-filters", "編譯時過濾器"),
            " 可以用於調整詳細程度或停用日誌記錄，這應該會導致更小的Wasm 檔案。",
        ],
        h2!["來源映射 (Source Maps)"],
        p![
            "有一些支援 ",
            link!("https://developer.chrome.com/blog/wasm-debugging-2019/#enter-dwarf", "來源映射"),
            "。但是，需要一些配置。",
        ],
        h2!["過去的文章"],
        p!["以下是一些關於 Rust 中 WebAssembly 偵錯狀態的過去文章。它們可能是有趣的閱讀。"],
        p![
            "\\[Dec 2019\\] ",
            link!("https://developers.google.com/web/updates/2019/12/webassembly#the_future", "Chrome DevTools 更新"),
        ],
        blockquote![p!["這些工作還有很多要做。例如，在工具方面，Emscripten（Binaryen）和 wasm-pack（wasm-bindgen）尚未支援在它們執行的轉換上更新 DWARF 資訊。"]],
        p![
            "\\[2020\\] ",
            link!("https://rustwasm.github.io/book/reference/debugging.html#using-a-debugger", "Rust Wasm 偵錯指南"),
        ],
        blockquote![p![
            "不幸的是，WebAssembly 的調試能力仍然不成熟。在大多數Unix 系統上，",
            link!("http://dwarfstd.org/", "DWARF"),
            " 用於編碼調試器需要提供運行中程序的源級檢查的信息，就連在Windows 上有一種編碼類似信息的替代格式。但目前，WebAssembly 並沒有對應的格式。",
        ]],
        p![
            "\\[2019\\] ",
            link!("https://rustwasm.github.io/rfcs/007-2019-roadmap.html#debugging", "Rust Wasm 路線圖"),
        ],
        blockquote![p!["偵錯很棘手，因為很多情況不在這個工作小組的掌控之中，而是取決於 WebAssembly 標準化機構和實現瀏覽器開發者工具的人。"]],
    ])
}

crate::doc_page!("調試", "/zh-Hant/docs/more/debugging", page_content());
