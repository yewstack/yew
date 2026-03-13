pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        blockquote(vec![p(vec![text(
            "Yew 在一个地方集中了一个可重用的 UI 部分可能需要的所有内容 - rust \
             文件，同时也在必要时保持底层技术的可访问性。",
        )])]),
        p(vec![text(
            "截至今天，WebAssembly 对于 DOM 交互还不完全支持。这意味着即使在 Yew \
             中，我们有时也依赖于调用 JavaScript。接下来是涉及的库的概述。",
        )]),
        h2(vec![text("wasm-bindgen")]),
        p(vec![
            link(
                "https://github.com/rustwasm/wasm-bindgen",
                vec![text("wasm-bindgen")],
            ),
            text(" 是一个在 JavaScript 和 Rust 函数之间建立调用桥梁的库和工具。"),
        ]),
        p(vec![
            text("我们强烈建议您查看他们的"),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/",
                vec![text("文档")],
            ),
            text("和我们的"),
            link(
                "/zh-Hans/docs/concepts/basic-web-technologies/wasm-bindgen",
                vec![text("快速指南")],
            ),
            text("。"),
        ]),
        h2(vec![text("web-sys")]),
        p(vec![
            link(
                "https://crates.io/crates/web-sys",
                vec![text("web-sys crate")],
            ),
            text(
                " 为 Web API 提供了绑定，并允许我们以一种经过 Rust 处理和安全的方式编写 \
                 JavaScript 代码。",
            ),
        ]),
        p(vec![text("示例：")]),
        tabs(
            "JS",
            vec![
                tab(
                    "JS",
                    "JS",
                    vec![code_block("js", "let document = window.document")],
                ),
                tab(
                    "RS",
                    "RS",
                    vec![code_block(
                        "rust",
                        r#"use wasm_bindgen::UnwrapThrowExt;
use web_sys::window;

let document = window()
    .expect_throw("window is undefined")
    .document()
    .expect_throw("document is undefined");"#,
                    )],
                ),
            ],
        ),
        p(vec![
            text("再次强调，我们强烈建议您查看他们的"),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/",
                vec![text("文档")],
            ),
            text("和我们的"),
            link(
                "/zh-Hans/docs/concepts/basic-web-technologies/web-sys",
                vec![text("快速指南")],
            ),
            text("。"),
        ]),
    ])
}

crate::doc_page!(
    "Javascript 与 Rust",
    "/zh-Hans/docs/concepts/basic-web-technologies/js",
    page_content()
);
