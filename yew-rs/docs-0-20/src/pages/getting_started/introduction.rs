crate::doc_page!("Getting Started", "/docs/getting-started",
    Content::new(vec![
        p(vec![
            text("You will need a couple of tools to compile, build, package and debug your Yew application. \
              When getting started, we recommend using "),
            link("https://trunkrs.dev/", vec![text("Trunk")]),
            text(". Trunk is a WASM web application bundler for Rust."),
        ]),
        h2(vec![text("Installing Rust")]),
        p(vec![
            text("To install Rust, follow the "),
            link("https://www.rust-lang.org/tools/install", vec![text("official instructions")]),
            text("."),
        ]),
        admonition(AdmonitionType::Warning, Some("Important"), vec![
            p(vec![
                text("The minimum supported Rust version (MSRV) for Yew is "),
                code("1.56.1"),
                text(". Older versions can cause unexpected issues accompanied by incomprehensible error messages. \
                  You can check your toolchain version using "),
                code("rustup show"),
                text(" (under \"active toolchain\") or alternatively "),
                code("rustc --version"),
                text(". To update your toolchain, run "),
                code("rustup update"),
                text("."),
            ]),
        ]),
        h2(vec![text("Install WebAssembly target")]),
        p(vec![
            text("Rust can compile source codes for different \"targets\" (e.g. different processors). The compilation \
              target for browser-based WebAssembly is called "),
            code("wasm32-unknown-unknown"),
            text(". The following command will add the WebAssembly target to your development environment."),
        ]),
        code_block("shell", "rustup target add wasm32-unknown-unknown"),
        h2(vec![text("Install Trunk")]),
        p(vec![
            text("Trunk is the recommended tool for managing deployment and packaging, and is used throughout the \
              documentation and examples."),
        ]),
        code_block("shell", "# note that this might take a while to install, because it compiles everything from scratch\n# Trunk also provides prebuilt binaries for a number of major package managers\n# See https://trunkrs.dev/#install for further details\ncargo install --locked trunk"),
        h3(vec![text("Other options")]),
        p(vec![
            text("There are options other than Trunk that may be used for bundling Yew applications. \
              You might want to try one of these options:"),
        ]),
        ul(vec![
            li(vec![link("https://github.com/drager/wasm-pack/", vec![text("wasm-pack")])]),
            li(vec![link("https://github.com/IMI-eRnD-Be/wasm-run", vec![text("wasm-run")])]),
            li(vec![link("https://github.com/rustminded/xtask-wasm/", vec![text("xtask-wasm")]), text(" (still in early development)")]),
        ]),
        h2(vec![text("Next steps")]),
        p(vec![
            text("With your development environment setup, you can now proceed with reading the documentation. \
              If you like to learn by getting your hands dirty, we recommend you check out our "),
            link("/docs/tutorial", vec![text("tutorial")]),
            text("."),
        ]),
    ])
);
