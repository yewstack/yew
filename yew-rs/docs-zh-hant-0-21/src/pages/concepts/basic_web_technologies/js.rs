crate::doc_page!(
    "Javascript 與 Rust",
    "/zh-Hant/docs/concepts/basic-web-technologies/js",
    Content::new(vec![
        p![text(
            "> Yew 在一個地方集中了一個可重用的 UI 部分可能需要的所有內容 - rust \
             文件，同時也在必要時保持底層技術的可訪問性。",
        )],
        p![text(
            "截至今天，WebAssembly 對於 DOM 互動還不完全支援。這意味著即使在 Yew \
             中，我們有時也依賴呼叫 JavaScript。接下來是涉及的庫的概述。",
        )],
        h2![text("wasm-bindgen")],
        p![
            link!(
                "https://github.com/rustwasm/wasm-bindgen",
                text("wasm-bindgen"),
            ),
            text(" 是一個在 JavaScript 和 Rust 函數之間建立呼叫橋樑的函式庫和工具。",),
        ],
        p![
            text("我們強烈建議您查看他們的"),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/",
                text("文件"),
            ),
            text("和我們的"),
            link!("", text("快速指南")),
            text("。"),
        ],
        h2![text("web-sys")],
        p![
            link!(
                "https://crates.io/crates/web-sys",
                text("web-sys crate"),
            ),
            text(
                " 為 Web API 提供了綁定，並允許我們以一種經過 Rust 處理和安全的方式編寫 \
                 JavaScript 程式碼。",
            ),
        ],
        p![text("範例：")],
        p![
            text("再次強調，我們強烈建議您查看他們的"),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/",
                text("文件"),
            ),
            text("和我們的"),
            link!("", text("快速指南")),
            text("。"),
        ],
    ])
);
