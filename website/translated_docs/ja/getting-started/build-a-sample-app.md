---
title: Build a sample app
---

はじめに、Rustの新規ライブラリを作りましょう(**重要:** `--lib`フラグを渡すことで_バイナリ_ではなく_ライブラリ_を作ってください)

```bash
cargo new --lib yew-app && cd yew-app
```

依存ライブラリに`yew`と`wasm-bindgen`を追加してください \(最新バージョンについては[こちら](https://docs.rs/yew)を参照してください\)

```toml title="Cargo.toml"
[package]
name = "yew-app"
version = "0.1.0"
authors = ["Yew App Developer <name@example.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
yew = "0.17"
wasm-bindgen = "0.2"
```

以下のテンプレートを `src/lib.rs`ファイルにコピーしてください:

```rust title="src/lib.rs"
use wasm_bindgen::prelude::*;
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
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
```

このテンプレートはルートに`Component`をセットアップし、`Model`と呼ばれるクリックしたら更新するボタンを作ります。
`main()`の中にある`App::<Model>::new().mount_to_body()`がアプリをスタートしてページの`<body>`タグをマウントすることに特に注意してください。
動的なプロパティでアプリをスタートしたい場合は代わりに`App::<Model>::new().mount_to_body_with_props(..)`を使うことで実現できます。

最後に、アプリの中の`static`という名前のフォルダに`index.html`ファイルを追加してください。

```bash
mkdir static
```

```markup title="index.html"
<!doctype html>
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
</html>
```

## アプリを動かす!

[`wasm-pack`](https://rustwasm.github.io/docs/wasm-pack/)を使うのがアプリを動かすのに推奨される方法です。
まだ`wasm-pack`をインストールしていない場合、`cargo install wasm-pack`でインストールして開発サーバーを動かしてみましょう:

```bash
wasm-pack build --target web --out-name wasm --out-dir ./static
```

`wasm-pack`はコンパイルされたWebAssemblyとJavaScriptラッパーをまとめたものを`./static`ディレクトリに作り、
アプリのWebAssemblyバイナリを読み込んで動かします。

そして、`./static`以下で好きなサーバーをファイルをサーブしてみましょう。
例えば:

```bash
cargo +nightly install miniserve
miniserve ./static --index index.html
```

