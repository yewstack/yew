crate::doc_page!(
    "JS with RS",
    "/zh-Hans/docs/concepts/basic-web-technologies/js",
    Content::new(vec![
        blockquote![p!["Yew centrally operates on the idea of keeping \
                        everything that a reusable piece of UI may need in \
                        one place - Rust files, while also keeping the \
                        underlying technology accessible where necessary."]],
        p![
            "As of today, WebAssembly is not feature-complete for DOM interactions. This means \
             even in Yew we sometimes rely on calling JavaScript. What follows is an overview of \
             the involved libraries."
        ],
        h2!["wasm-bindgen"],
        p![
            link!(
                "https://github.com/rustwasm/wasm-bindgen",
                code("wasm-bindgen")
            ),
            " is a library and tool that bridges calls between JavaScript and Rust functions."
        ],
        p![
            "We highly recommend you take a look at their ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/",
                "documentation"
            ),
            " and our ",
            link!(
                "/docs/0.21/concepts/basic-web-technologies/wasm-bindgen",
                "quick guide"
            ),
            "."
        ],
        h2!["web-sys"],
        p![
            "The ",
            code("web-sys"),
            " crate",
            " provides bindings for Web APIs and allows us to write JavaScript code in a \
             rustyfied and safe way."
        ],
        p!["Example:"],
        h3!["JS"],
        code_block("js", "let document = window.document"),
        h3!["RS"],
        code_block(
            "rust",
            "use wasm_bindgen::UnwrapThrowExt;
use web_sys::window;

let document = window()
    .expect_throw(\"window is undefined\")
    .document()
    .expect_throw(\"document is undefined\");"
        ),
        p![
            "Once again we highly recommend you take a look at their ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/",
                "documentation"
            ),
            " and our ",
            link!(
                "/docs/0.21/concepts/basic-web-technologies/web-sys",
                "quick guide"
            ),
            "."
        ]
    ])
    .with_description("JavaScript with Rust")
);
