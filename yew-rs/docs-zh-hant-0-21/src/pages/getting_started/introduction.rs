crate::doc_page!(
    "開始使用",
    "/zh-Hant/docs/getting-started",
    Content::new(vec![
        p![
            "你需要一些工具來編譯、建置、打包和調試你的 Yew 應用程式。在最開始，我們建議使用 ",
            link!("https://trunkrs.dev/", "Trunk"),
            "。 Trunk 是用於 Rust 的 WASM Web 應用程式打包工具。",
        ],
        h2!["安裝 Rust"],
        p![
            "要安裝 Rust，請按照",
            link!("https://www.rust-lang.org/tools/install", "官方說明",),
            "。",
        ],
        admonition!(
            AdmonitionType::Important,
            None,
            p![
                "Yew 支援的最低 Rust 版本（MSRV）是 ",
                code("1.84.0"),
                "。舊版將無法編譯。您可以使用 ",
                code("rustup show"),
                "（在「active toolchain」下）或 ",
                code("rustc --version"),
                " 檢查您的工具鏈版本。若要更新您的工具鏈，請執行 ",
                code("rustup update"),
                "。",
            ],
        ),
        h2!["安裝 WebAssembly 目標"],
        p![
            "Rust 可以為不同的「目標」（例如不同的處理器）編譯原始碼。用於基於瀏覽器的 \
             WebAssembly 的編譯目標稱為 ",
            code("wasm32-unknown-unknown"),
            "。以下命令將向您的開發環境新增 WebAssembly 目標。",
        ],
        code_block("shell", r#"rustup target add wasm32-unknown-unknown"#),
        h2!["安裝 Trunk"],
        p!["Trunk 是建議的管理部署和包裝的工具，並在整個文件和範例中使用。"],
        code_block(
            "shell",
            r#"# 需要注意的是，這可能需要一段時間來安裝，因為它會從頭開始編譯所有內容
# Trunk 也為許多主要的套件管理器提供了預先建置的二進位文件
# 有關更多詳細信息，請參見 https://trunkrs.dev/#install
cargo install --locked trunk"#,
        ),
        h3!["其他選項"],
        p!["除了 Trunk 之外，還有其他選項可用於打包 Yew 應用程式。您可能想嘗試以下選項之一："],
        ul![
            li![link!("https://github.com/drager/wasm-pack/", "wasm-pack",)],
            li![link!("https://github.com/IMI-eRnD-Be/wasm-run", "wasm-run",)],
            li![
                link!("https://github.com/rustminded/xtask-wasm/", "xtask-wasm",),
                " (仍在早期開發階段)",
            ],
        ],
        h2!["下一步"],
        p![
            "設定好開發環境後，現在可以繼續閱讀文件。如果您喜歡透過動手實作來學習，\
             我們建議您查看我們的",
            link!("", "教學"),
            "。",
        ],
    ])
);
