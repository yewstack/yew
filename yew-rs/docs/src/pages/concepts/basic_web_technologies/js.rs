pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        blockquote(vec![p(vec![text(
            "Yew centrally operates on the idea of keeping everything that a reusable piece of UI \
             may need in one place - rust files, while also keeping the underlying technology \
             accessible where necessary.",
        )])]),
        p(vec![text(
            "As of today, WebAssembly is not feature-complete for DOM interactions. This means \
             even in Yew we sometimes rely on calling JavaScript. What follows is an overview of \
             the involved libraries.",
        )]),
        h2(vec![text("wasm-bindgen")]),
        p(vec![
            link(
                "https://github.com/rustwasm/wasm-bindgen",
                vec![code("wasm-bindgen")],
            ),
            text(
                " is a library and tool that bridges calls between JavaScript and Rust functions.",
            ),
        ]),
        p(vec![
            text("We highly recommend you take a look at their "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/",
                vec![text("documentation")],
            ),
            text(" and our "),
            link(
                "/docs/concepts/basic-web-technologies/wasm-bindgen",
                vec![text("quick guide")],
            ),
            text("."),
        ]),
        h2(vec![text("web-sys")]),
        p(vec![
            text("The "),
            link(
                "https://crates.io/crates/web-sys",
                vec![code("web-sys"), text(" crate")],
            ),
            text(
                " provides bindings for Web APIs and allows us to write JavaScript code in a \
                 rustyfied and safe way.",
            ),
        ]),
        p(vec![text("Example:")]),
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
                    vec![code_block_no_run(
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
            text("Once again we highly recommend you take a look at their "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/",
                vec![text("documentation")],
            ),
            text(" and our "),
            link(
                "/docs/concepts/basic-web-technologies/web-sys",
                vec![text("quick guide")],
            ),
            text("."),
        ]),
    ])
}

crate::doc_page!(
    "JS with RS",
    "/docs/concepts/basic-web-technologies/js",
    page_content()
);
