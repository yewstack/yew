pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        blockquote![p![text(
            "Yew は、再利用可能な UI \
             部分に必要なすべてのコンテンツを1か所に集める一方で、\
             必要に応じて基盤技術へのアクセスも維持します。",
        )]],
        p![text(
            "今日現在、WebAssembly は DOM との相互作用を完全にはサポートしていません。これは、Yew \
             でも時々 JavaScript \
             の呼び出しに依存することを意味します。次に、関係するライブラリの概要を示します。",
        )],
        h2![text("wasm-bindgen")],
        p![
            link!(
                "https://github.com/rustwasm/wasm-bindgen",
                code("wasm-bindgen"),
            ),
            text(" は、JavaScript と Rust 関数の間に呼び出しの橋を架けるライブラリとツールです。"),
        ],
        p![
            text("彼らの"),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/",
                text("ドキュメント"),
            ),
            text("と私たちの"),
            link!(
                "/ja/docs/concepts/basic-web-technologies/wasm-bindgen",
                text("クイックガイド"),
            ),
            text("を強くお勧めします。"),
        ],
        h2![text("web-sys")],
        p![
            link!(
                "https://crates.io/crates/web-sys",
                code("web-sys"),
                text(" crate"),
            ),
            text(
                " は Web API のバインディングを提供し、Rust で処理され安全な方法で JavaScript \
                 コードを書くことを可能にします。",
            ),
        ],
        p![text("例：")],
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
        p![
            text("繰り返しになりますが、彼らの"),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/",
                text("ドキュメント"),
            ),
            text("と私たちの"),
            link!(
                "/ja/docs/concepts/basic-web-technologies/web-sys",
                text("クイックガイド"),
            ),
            text("を強くお勧めします。"),
        ],
    ])
}

crate::doc_page!(
    "JavaScript と Rust",
    "/ja/docs/concepts/basic-web-technologies/js",
    page_content()
);
