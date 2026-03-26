crate::doc_page!(
    "Build a sample app",
    "/ja/docs/getting-started/build-a-sample-app",
    Content::new(vec![
        p![
            "はじめに、Rustの新規ライブラリを作りましょう（",
            bold!["重要:"],
            " ",
            code("--lib"),
            "フラグを渡すことで",
            italic!["バイナリ"],
            "ではなく",
            italic!["ライブラリ"],
            "を作ってください）",
        ],
        code_block("bash", "cargo new --lib yew-app && cd yew-app"),
        p![
            "依存ライブラリに",
            code("yew"),
            "と",
            code("wasm-bindgen"),
            "を追加してください（最新バージョンについては",
            link!("https://docs.rs/yew", "こちら"),
            "を参照してください）",
        ],
        code_block_title(
            "toml",
            "Cargo.toml",
            r#"[package]
name = "yew-app"
version = "0.1.0"
authors = ["Yew App Developer <name@example.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
yew = "0.17"
wasm-bindgen = "0.2""#
        ),
        p![
            "以下のテンプレートを ",
            code("src/lib.rs"),
            "ファイルにコピーしてください:",
        ],
        code_block_title(
            "rust",
            "src/lib.rs",
            r#"use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick={self.link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}"#
        ),
        p![
            "このテンプレートはルートに",
            code("Component"),
            "をセットアップし、",
            code("Model"),
            "と呼ばれるクリックしたら更新するボタンを作ります。",
        ],
        p![
            code("main()"),
            "の中にある",
            code("App::<Model>::new().mount_to_body()"),
            "がアプリをスタートしてページの",
            code("<body>"),
            "タグをマウントすることに特に注意してください。\
             動的なプロパティでアプリをスタートしたい場合は代わりに",
            code("App::<Model>::new().mount_to_body_with_props(..)"),
            "を使うことで実現できます。",
        ],
        p![
            "最後に、アプリの中の",
            code("static"),
            "という名前のフォルダに",
            code("index.html"),
            "ファイルを追加してください。",
        ],
        code_block("bash", "mkdir static"),
        code_block_title(
            "html",
            "index.html",
            r#"<!doctype html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Yew Sample App</title>
        <script type="module">
            import init from "./wasm.js"
            init()
        </script>
    </head>
    <body></body>
</html>"#
        ),
        h2!["アプリを動かす!"],
        p![
            link!(
                "https://drager.github.io/wasm-pack/book/",
                code("wasm-pack")
            ),
            "を使うのがアプリを動かすのに推奨される方法です。まだ",
            code("wasm-pack"),
            "をインストールしていない場合、",
            code("cargo install wasm-pack"),
            "でインストールして開発サーバーを動かしてみましょう:",
        ],
        code_block(
            "bash",
            "wasm-pack build --target web --out-name wasm --out-dir ./static"
        ),
        p![
            code("wasm-pack"),
            "はコンパイルされたWebAssemblyとJavaScriptラッパーをまとめたものを",
            code("./static"),
            "ディレクトリに作り、アプリのWebAssemblyバイナリを読み込んで動かします。",
        ],
        p![
            "そして、",
            code("./static"),
            "以下で好きなサーバーをファイルをサーブしてみましょう。例えば:",
        ],
        code_block(
            "bash",
            "cargo +nightly install miniserve\nminiserve ./static --index index.html"
        ),
    ])
);
