crate::doc_page!(
    "",
    "/zh-Hans/docs/more/debugging",
    Content::new(vec![
        h1![text("Debugging")],
        h2![text("Panics")],
        p![
            text("Please use the "),
            link!(
                "https://github.com/rustwasm/console_error_panic_hook",
                code("console_error_panic")
            ),
            text(
                " crate for nicer stacktraces with Rust symbols. Note, that it is not compatible \
                 with apps built with "
            ),
            code("cargo-web"),
            text(".")
        ],
        h2![text("Console Logging")],
        p![
            text("In general, Wasm web apps are able to interact with Browser APIs, and the "),
            code("console.log"),
            text(" api is no exception. There are a few options available:")
        ],
        h3![link!(
            "https://crates.io/crates/wasm-logger",
            code("wasm-logger")
        )],
        p![
            text("This crate integrates with the familiar Rust "),
            code("log"),
            text(" crate:")
        ],
        code_block(
            "rust",
            r#"// setup
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
}

// usage
log::info!("Update: {:?}", msg);"#
        ),
        h3![link!(
            "https://docs.rs/yew/0.13.2/yew/services/console/struct.ConsoleService.html",
            code("ConsoleService")
        )],
        p![
            text("This service is included within yew and is available when the "),
            code("\"services\""),
            text(" feature is enabled:")
        ],
        code_block(
            "rust",
            r#"// usage
ConsoleService::info(format!("Update: {:?}", msg).as_ref());"#
        ),
        h2![text("Source Maps")],
        p![text(
            "There is currently no first-class support for source maps for Rust / Wasm web apps. \
             This, of course, is subject to change. If this is no longer true or if progress is \
             made, please suggest a change!"
        )],
        h3![text("Latest Info")],
        p![
            text("[Dec 2019] "),
            link!(
                "https://developers.google.com/web/updates/2019/12/webassembly#the_future",
                text("Chrome DevTools update")
            )
        ],
        blockquote![p![text(
            "There is still quite a bit of work to do though. For example, on the tooling side, \
             Emscripten (Binaryen) and wasm-pack (wasm-bindgen) don't support updating DWARF \
             information on transformations they perform yet."
        )]],
        p![
            text("[2020] "),
            link!(
                "https://rustwasm.github.io/book/reference/debugging.html#using-a-debugger",
                text("Rust Wasm debugging guide")
            )
        ],
        blockquote![p![
            text(
                "Unfortunately, the debugging story for WebAssembly is still immature. On most \
                 Unix systems, "
            ),
            link!("http://dwarfstd.org/", text("DWARF")),
            text(
                " is used to encode the information that a debugger needs to provide source-level \
                 inspection of a running program. There is an alternative format that encodes \
                 similar information on Windows. Currently, there is no equivalent for \
                 WebAssembly."
            )
        ]],
        p![
            text("[2019] "),
            link!(
                "https://rustwasm.github.io/rfcs/007-2019-roadmap.html#debugging",
                text("Rust Wasm roadmap")
            )
        ],
        blockquote![p![text(
            "Debugging is tricky because much of the story is out of this working group's hands, \
             and depends on both the WebAssembly standardization bodies and the folks \
             implementing browser developer tools instead."
        )]]
    ])
);
