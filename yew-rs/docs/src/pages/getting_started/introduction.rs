pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            text("You will need a couple of tools to compile, build, package, and debug your Yew application. \
              When getting started, we recommend using "),
            link!("https://trunkrs.dev/", text("Trunk")),
            text(". Trunk is a Wasm web application bundler for Rust."),
        ],
        h2![text("Install Rust")],
        p![
            text("To install Rust, follow the "),
            link!("https://www.rust-lang.org/tools/install", text("official instructions")),
            text("."),
        ],
        admonition!(AdmonitionType::Important, None,
            p![
                text("The minimum supported Rust version (MSRV) for Yew is "),
                code("1.84.0"),
                text(". Older versions will not compile. You can check your toolchain version using "),
                code("rustup show"),
                text(" (under \"active toolchain\") or "),
                code("rustc --version"),
                text(". To update your toolchain, run "),
                code("rustup update"),
                text("."),
            ],
        ),
        h2![text("Install WebAssembly target")],
        p![
            text("Rust can compile source codes for different \"targets\" (e.g. different processors). The compilation \
              target for browser-based WebAssembly is called "),
            code("wasm32-unknown-unknown"),
            text(". The following command will add the WebAssembly target to your development environment."),
        ],
        code_block("shell", "rustup target add wasm32-unknown-unknown"),
        h2![text("Install Trunk")],
        p![
            text("Trunk is the recommended tool for managing deployment and packaging and is used throughout the \
              documentation and examples."),
        ],
        code_block("shell",
"# note that this might take a while to install because it compiles everything from scratch
# Trunk also provides prebuilt binaries for a number of major package managers
# See https://trunkrs.dev/#install for further details
cargo install --locked trunk"
        ),
        h3![text("Other options")],
        p![
            text("There are options other than Trunk that may be used for bundling Yew applications. \
              You might want to try one of these options:"),
        ],
        ul![
            li![link!("https://github.com/drager/wasm-pack/", text("wasm-pack"))],
            li![link!("https://github.com/IMI-eRnD-Be/wasm-run", text("wasm-run"))],
            li![link!("https://github.com/rustminded/xtask-wasm/", text("xtask-wasm")), text(" (still in early development)")],
        ],
        h2![text("Next steps")],
        p![
            text("With your development environment set up, you can now either proceed with reading the documentation, or \
              if you like to learn by getting your hands dirty, we recommend you check out our "),
            link!("/docs/tutorial", text("tutorial")),
            text("."),
        ],
    ])
}

crate::doc_page!("Getting Started", "/docs/getting-started", page_content());
