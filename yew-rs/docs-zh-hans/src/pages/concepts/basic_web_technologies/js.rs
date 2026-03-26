pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        blockquote![p!["Yew 在一个地方集中了一个可重用的 UI \
                        部分可能需要的所有内容 - rust \
                        文件，同时也在必要时保持底层技术的可访问性。"]],
        p![
            "截至今天，WebAssembly 对于 DOM 交互还不完全支持。这意味着即使在 Yew \
             中，我们有时也依赖于调用 JavaScript。接下来是涉及的库的概述。"
        ],
        h2!["wasm-bindgen"],
        p![
            link!("https://github.com/rustwasm/wasm-bindgen", "wasm-bindgen",),
            " 是一个在 JavaScript 和 Rust 函数之间建立调用桥梁的库和工具。",
        ],
        p![
            "我们强烈建议您查看他们的",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/", "文档",),
            "和我们的",
            link!(
                "/zh-Hans/docs/concepts/basic-web-technologies/wasm-bindgen",
                "快速指南",
            ),
            "。",
        ],
        h2!["web-sys"],
        p![
            link!("https://crates.io/crates/web-sys", "web-sys crate",),
            " 为 Web API 提供了绑定，并允许我们以一种经过 Rust 处理和安全的方式编写 JavaScript \
             代码。",
        ],
        p!["示例："],
        tabs![
            "JS",
            tab![
                "JS",
                "JS",
                code_block("js", "let document = window.document"),
            ],
            tab![
                "RS",
                "RS",
                code_block(
                    "rust",
                    r#"use wasm_bindgen::UnwrapThrowExt;
use web_sys::window;

let document = window()
    .expect_throw("window is undefined")
    .document()
    .expect_throw("document is undefined");"#,
                ),
            ],
        ],
        p![
            "再次强调，我们强烈建议您查看他们的",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/", "文档",),
            "和我们的",
            link!(
                "/zh-Hans/docs/concepts/basic-web-technologies/web-sys",
                "快速指南",
            ),
            "。",
        ],
    ])
}

crate::doc_page!(
    "Javascript 与 Rust",
    "/zh-Hans/docs/concepts/basic-web-technologies/js",
    page_content()
);
