pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("意外终止 (Panics)")],
        p![text("Yew 会自动在浏览器控制台中输出意外终止日志。")],
        h2![text("控制台日志")],
        p![
            text("在 JavaScript 中，"),
            code("console.log()"),
            text(" 用于输出到浏览器控制台。以下是一些 Yew 的选项。"),
        ],
        h3![link!("https://crates.io/crates/wasm-logger", code("wasm-logger"))],
        p![
            code("wasm-logger"),
            text(" crate 与 "),
            link!("https://crates.io/crates/log", code("log")),
            text(" crate 集成，以将日志级别、源行和文件名发送到浏览器控制台。"),
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
            text("这个 crate 是 Gloo 的一部分，提供了对浏览器 API 的 Rust 包装。"),
            code("log!"),
            text(" 宏可以直接接受 "),
            code("JsValue"),
            text("，比 "),
            code("wasm_logger"),
            text(" 更容易使用。"),
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
            text(" 可以与 "),
            link!("https://crates.io/crates/tracing-subscriber", code("tracing-subscriber")),
            text(" 一起使用，将消息输出到浏览器控制台。"),
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
        h2![text("调试组件生命周期")],
        p![
            link!("https://crates.io/crates/tracing", code("tracing")),
            text(" 可以用于收集与组件生命周期相关的事件信息。"),
            code("tracing"),
            text(" 还带有一个 "),
            code("log"),
            text(" 支持的特性标志，可以与 "),
            code("wasm-logger"),
            text(" 很好地集成。"),
        ],
        p![
            link!("https://docs.rs/tracing/latest/tracing/level_filters/index.html#compile-time-filters", text("编译时过滤器")),
            text(" 可以用于调整详细程度或禁用日志记录，这应该会导致更小的 Wasm 文件。"),
        ],
        h2![text("源映射 (Source Maps)")],
        p![
            text("有一些支持 "),
            link!("https://developer.chrome.com/blog/wasm-debugging-2019/#enter-dwarf", text("源映射")),
            text("。但是，需要一些配置。"),
        ],
        h2![text("过去的文章")],
        p![text("以下是一些关于 Rust 中 WebAssembly 调试状态的过去文章。它们可能是有趣的阅读。")],
        p![
            text("\\[Dec 2019\\] "),
            link!("https://developers.google.com/web/updates/2019/12/webassembly#the_future", text("Chrome DevTools 更新")),
        ],
        blockquote![p![text("这些工作还有很多要做。例如，在工具方面，Emscripten（Binaryen）和 wasm-pack（wasm-bindgen）尚未支持在它们执行的转换上更新 DWARF 信息。")]],
        p![
            text("\\[2020\\] "),
            link!("https://rustwasm.github.io/book/reference/debugging.html#using-a-debugger", text("Rust Wasm 调试指南")),
        ],
        blockquote![p![
            text("不幸的是，WebAssembly 的调试能力仍然不成熟。在大多数 Unix 系统上，"),
            link!("http://dwarfstd.org/", text("DWARF")),
            text(" 用于编码调试器需要提供运行中程序的源级检查的信息，就连在 Windows 上有一种编码类似信息的替代格式。但目前，WebAssembly 没有相应的格式。"),
        ]],
        p![
            text("\\[2019\\] "),
            link!("https://rustwasm.github.io/rfcs/007-2019-roadmap.html#debugging", text("Rust Wasm 路线图")),
        ],
        blockquote![p![text("调试很棘手，因为很多情况不在这个工作组的掌控之中，而是取决于 WebAssembly 标准化机构和实现浏览器开发者工具的人。")]],
    ])
}

crate::doc_page!("调试", "/zh-Hans/docs/more/debugging", page_content());
