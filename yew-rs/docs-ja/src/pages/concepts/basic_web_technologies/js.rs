pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        blockquote![p!["Yew は、再利用可能な UI \
                        部分に必要なすべてのコンテンツを1か所に集める一方で、\
                        必要に応じて基盤技術へのアクセスも維持します。"]],
        p![
            "今日現在、WebAssembly は DOM との相互作用を完全にはサポートしていません。これは、Yew \
             でも時々 JavaScript \
             の呼び出しに依存することを意味します。次に、関係するライブラリの概要を示します。"
        ],
        h2!["wasm-bindgen"],
        p![
            link!(
                "https://github.com/rustwasm/wasm-bindgen",
                code("wasm-bindgen"),
            ),
            " は、JavaScript と Rust 関数の間に呼び出しの橋を架けるライブラリとツールです。",
        ],
        p![
            "彼らの",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/",
                "ドキュメント",
            ),
            "と私たちの",
            doc_link!(
                crate::pages::concepts::basic_web_technologies::wasm_bindgen,
                "クイックガイド",
            ),
            "を強くお勧めします。",
        ],
        h2!["web-sys"],
        p![
            link!(
                "https://crates.io/crates/web-sys",
                code("web-sys"),
                " crate",
            ),
            " は Web API のバインディングを提供し、Rust で処理され安全な方法で JavaScript \
             コードを書くことを可能にします。",
        ],
        p!["例："],
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
            "繰り返しになりますが、彼らの",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/",
                "ドキュメント",
            ),
            "と私たちの",
            doc_link!(
                crate::pages::concepts::basic_web_technologies::web_sys,
                "クイックガイド",
            ),
            "を強くお勧めします。",
        ],
    ])
    .with_description("JavaScript with Rust")
}

crate::doc_page!(
    "JavaScript と Rust",
    "/ja/docs/concepts/basic-web-technologies/js",
    page_content()
);
