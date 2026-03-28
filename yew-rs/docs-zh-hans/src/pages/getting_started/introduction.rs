pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "你需要一些工具来编译、构建、打包和调试你的 Yew 应用程序。在最开始，我们建议使用 ",
            link!("https://trunkrs.dev/", "Trunk"),
            "。Trunk 是一个用于 Rust 的 WASM Web 应用程序打包工具。",
        ],
        h2!["安装 Rust"],
        p![
            "要安装 Rust，请按照",
            link!("https://www.rust-lang.org/tools/install", "官方说明",),
            "。",
        ],
        admonition!(
            AdmonitionType::Important,
            None,
            p![
                "Yew 支持的最低 Rust 版本（MSRV）是 ",
                code("1.84.0"),
                "。旧版本将无法编译。您可以使用 ",
                code("rustup show"),
                "（在\"active toolchain\"下）或 ",
                code("rustc --version"),
                " 检查您的工具链版本。要更新您的工具链，请运行 ",
                code("rustup update"),
                "。",
            ],
        ),
        h2!["安装 WebAssembly 目标"],
        p![
            "Rust 可以为不同的\"目标\"（例如不同的处理器）编译源代码。用于基于浏览器的 \
             WebAssembly 的编译目标称为 ",
            code("wasm32-unknown-unknown"),
            "。以下命令将向您的开发环境添加 WebAssembly 目标。",
        ],
        code_block("shell", r#"rustup target add wasm32-unknown-unknown"#),
        h2!["安装 Trunk"],
        p!["Trunk 是推荐的用于管理部署和打包的工具，并在整个文档和示例中使用。"],
        code_block(
            "shell",
            r#"# 需要注意的是，这可能需要一段时间来安装，因为它会从头开始编译所有内容
# Trunk 还为许多主要的包管理器提供了预构建的二进制文件
# 有关更多详细信息，请参见 https://trunkrs.dev/#install
cargo install --locked trunk"#,
        ),
        h3!["其他选项"],
        p!["除了 Trunk 之外，还有其他选项可用于打包 Yew 应用程序。您可能想尝试以下选项之一："],
        ul![
            li![link!(
                "https://github.com/drager/wasm-pack/",
                code("wasm-pack"),
            )],
            li![link!(
                "https://github.com/IMI-eRnD-Be/wasm-run",
                code("wasm-run"),
            )],
            li![
                link!(
                    "https://github.com/rustminded/xtask-wasm/",
                    code("xtask-wasm"),
                ),
                " (仍在早期开发阶段)",
            ],
        ],
        h2!["下一步"],
        p![
            "设置好开发环境后，您现在可以继续阅读文档。如果您喜欢通过动手实践来学习，\
             我们建议您查看我们的",
            doc_link!(@ "/tutorial", "教程"),
            "。",
        ],
    ])
}

crate::doc_page!("开始使用", "/zh-Hans/docs/getting-started", page_content());
