crate::doc_page!(
    "Getting Started",
    "/zh-Hans/docs/getting-started",
    Content::new(vec![
        p![
            "You will need a couple of tools to compile, build, package and debug your Yew application. \
              When getting started, we recommend using ",
            link!["https://trunkrs.dev/", "Trunk"],
            ". Trunk is a WASM web application bundler for Rust.",
        ],
        h2!["Installing Rust"],
        p![
            "To install Rust, follow the ",
            link!["https://www.rust-lang.org/tools/install", "official instructions"],
            ".",
        ],
        admonition![
            AdmonitionType::Warning,
            Some("Important"),
            p![
                "The minimum supported Rust version (MSRV) for Yew is ",
                code("1.56.1"),
                ". Older versions can cause unexpected issues accompanied by incomprehensible error messages. \
                  You can check your toolchain version using ",
                code("rustup show"),
                " (under \"active toolchain\") or alternatively ",
                code("rustc --version"),
                ". To update your toolchain, run ",
                code("rustup update"),
                ".",
            ],
        ],
        h2!["Install WebAssembly target"],
        p![
            "Rust can compile source codes for different \"targets\" (e.g. different processors). The compilation \
              target for browser-based WebAssembly is called ",
            code("wasm32-unknown-unknown"),
            ". The following command will add the WebAssembly target to your development environment.",
        ],
        code_block("shell", "rustup target add wasm32-unknown-unknown"),
        h2!["Install Trunk"],
        p![
            "Trunk is the recommended tool for managing deployment and packaging, and is used throughout the \
              documentation and examples.",
        ],
        code_block("shell",
"# note that this might take a while to install, because it compiles everything from scratch
# Trunk also provides prebuilt binaries for a number of major package managers
# See https://trunkrs.dev/#install for further details
cargo install --locked trunk"),
        h3!["Other options"],
        p![
            "There are options other than Trunk that may be used for bundling Yew applications. \
              You might want to try one of these options:",
        ],
        ul![
            li![link!["https://github.com/drager/wasm-pack/", "wasm-pack"]],
            li![link!["https://github.com/IMI-eRnD-Be/wasm-run", "wasm-run"]],
            li![
                link!["https://github.com/rustminded/xtask-wasm/", "xtask-wasm"],
                " (still in early development)",
            ],
        ],
        h2!["Next steps"],
        p![
            "With your development environment setup, you can now proceed with reading the documentation. \
              If you like to learn by getting your hands dirty, we recommend you check out our ",
            link!["/docs/0.20/tutorial", "tutorial"],
            ".",
        ],
    ])
);
