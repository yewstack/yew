crate::doc_page!("Debugging", "/docs/more/debugging", {
    Content::new(vec![
        h2![text("Panics")],
        p![text("Yew automatically logs panics in the browser console.")],
        h2![text("Console Logging")],
        p![
            text("In JavaScript, "),
            code("console.log()"),
            text(" is used to log to the browser console. Some options for Yew are listed below."),
        ],
        h3![
            link!("https://crates.io/crates/wasm-logger", code("wasm-logger")),
        ],
        p![
            code("wasm-logger"),
            text(" crate integrates with "),
            link!("https://crates.io/crates/log", code("log")),
            text(" crate to send the log level, source line, and filename to the browser console."),
        ],
        code_block("rust", r#"use log::info;
use wasm_bindgen::JsValue;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let object = JsValue::from("world");
    info!("Hello {}", object.as_string().unwrap());
}"#),
        h3![
            link!("https://crates.io/crates/gloo-console", code("gloo-console")),
        ],
        p![
            text("This crate is part of Gloo, a collection of libraries providing ergonomic Rust wrappers for browser APIs. \
              The "),
            code("log!"),
            text(" macro can take a "),
            code("JsValue"),
            text(" directly which is slightly easier to use than "),
            code("wasm_logger"),
            text("."),
        ],
        code_block("rust", r#"use gloo_console::log;
use wasm_bindgen::JsValue;

fn main() {
    let object = JsValue::from("world");
    log!("Hello", object)
}"#),
        h3![
            link!("https://crates.io/crates/tracing-web", code("tracing-web")),
        ],
        p![
            code("tracing-web"),
            text(" can be used with "),
            link!("https://crates.io/crates/tracing-subscriber", code("tracing-subscriber")),
            text(" to output messages to the browser console."),
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
        h2![text("Debugging component lifecycles")],
        p![
            link!("https://crates.io/crates/tracing", code("tracing")),
            text(" can be used to collect event information related to a component's lifecycle. "),
            code("tracing"),
            text(" also comes with a feature flag for "),
            code("log"),
            text(" support, which integrates nicely with "),
            code("wasm-logger"),
            text("."),
        ],
        p![
            link!("https://docs.rs/tracing/latest/tracing/level_filters/index.html#compile-time-filters", text("Compile time filters")),
            text(" can be used to adjust verbosity or disable logging, which should result in a smaller Wasm file."),
        ],
        h2![text("Source Maps")],
        p![
            text("There is "),
            link!("https://developer.chrome.com/blog/wasm-debugging-2019/#enter-dwarf", text("some support")),
            text(" for source maps. However, some configuration is required."),
        ],
        h2![text("Past Articles")],
        p![
            text("Some past articles on the state of debugging in WebAssembly in Rust can be found below. \
              They may serve as interesting reads."),
        ],
        p![
            text("[Dec 2019] "),
            link!("https://developers.google.com/web/updates/2019/12/webassembly#the_future", text("Chrome DevTools update")),
        ],
        blockquote![
            p![
                text("There is still quite a bit of work to do though. For example, on the tooling side, \
                  Emscripten (Binaryen) and wasm-pack (wasm-bindgen) does not support updating DWARF \
                  information on transformations they perform yet."),
            ],
        ],
        p![
            text("[2020] "),
            link!("https://rustwasm.github.io/book/reference/debugging.html#using-a-debugger", text("Rust Wasm debugging guide")),
        ],
        blockquote![
            p![
                text("Unfortunately, the debugging story for WebAssembly is still immature. On most Unix systems, "),
                link!("http://dwarfstd.org/", text("DWARF")),
                text(" is used to encode the information that a debugger needs to provide source-level inspection \
                  of a running program. There is an alternative format that encodes similar information on Windows. \
                  Currently, there is no equivalent for WebAssembly."),
            ],
        ],
        p![
            text("[2019] "),
            link!("https://rustwasm.github.io/rfcs/007-2019-roadmap.html#debugging", text("Rust Wasm roadmap")),
        ],
        blockquote![
            p![
                text("Debugging is tricky because much of the story is out of this working group's hands, \
                  and depends on both the WebAssembly standardization bodies and the folks implementing \
                  browser developer tools instead."),
            ],
        ],
    ])
});
