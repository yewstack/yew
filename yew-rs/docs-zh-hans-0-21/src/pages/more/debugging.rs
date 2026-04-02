crate::doc_page!(
    "",
    "/zh-Hans/docs/more/debugging",
    Content::new(vec![
        h1!["Debugging"],
        h2!["Panics"],
        p![
            "Please use the ",
            link!(
                "https://github.com/rustwasm/console_error_panic_hook",
                code("console_error_panic")
            ),
            " crate for nicer stacktraces with Rust symbols. Note, that it is not compatible with \
             apps built with ",
            code("cargo-web"),
            "."
        ],
        h2!["Console Logging"],
        p![
            "In general, Wasm web apps are able to interact with Browser APIs, and the ",
            code("console.log"),
            " api is no exception. There are a few options available:"
        ],
        h3![link!(
            "https://crates.io/crates/wasm-logger",
            code("wasm-logger")
        )],
        p![
            "This crate integrates with the familiar Rust ",
            code("log"),
            " crate:"
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
            "This service is included within yew and is available when the ",
            code("\"services\""),
            " feature is enabled:"
        ],
        code_block(
            "rust",
            r#"// usage
ConsoleService::info(format!("Update: {:?}", msg).as_ref());"#
        ),
        h2!["Source Maps"],
        p![
            "There is currently no first-class support for source maps for Rust / Wasm web apps. \
             This, of course, is subject to change. If this is no longer true or if progress is \
             made, please suggest a change!"
        ],
        h3!["Latest Info"],
        p![
            "[Dec 2019] ",
            link!(
                "https://developers.google.com/web/updates/2019/12/webassembly#the_future",
                "Chrome DevTools update"
            )
        ],
        blockquote![p!["There is still quite a bit of work to do though. For \
                        example, on the tooling side, Emscripten (Binaryen) \
                        and wasm-pack (wasm-bindgen) don't support updating \
                        DWARF information on transformations they perform yet."]],
        p![
            "[2020] ",
            link!(
                "https://rustwasm.github.io/book/reference/debugging.html#using-a-debugger",
                "Rust Wasm debugging guide"
            )
        ],
        blockquote![p![
            "Unfortunately, the debugging story for WebAssembly is still immature. On most Unix \
             systems, ",
            link!("http://dwarfstd.org/", "DWARF"),
            " is used to encode the information that a debugger needs to provide source-level \
             inspection of a running program. There is an alternative format that encodes similar \
             information on Windows. Currently, there is no equivalent for WebAssembly."
        ]],
        p![
            "[2019] ",
            link!(
                "https://rustwasm.github.io/rfcs/007-2019-roadmap.html#debugging",
                "Rust Wasm roadmap"
            )
        ],
        blockquote![p!["Debugging is tricky because much of the story is out \
                        of this working group's hands, and depends on both \
                        the WebAssembly standardization bodies and the folks \
                        implementing browser developer tools instead."]]
    ])
);
