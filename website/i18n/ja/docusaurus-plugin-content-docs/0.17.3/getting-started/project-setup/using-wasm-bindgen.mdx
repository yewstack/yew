---
title: Using wasm-bindgen
---

## インストール

```bash
cargo install wasm-bindgen-cli
```

## ビルド

はじめに、Wasmファイルを生成するアプリをビルドしましょう。
[サンプルアプリをビルド](../build-a-sample-app.md)のアプリをビルドしたいとします。
生成されたファイルのパスは`target/wasm32-unknown-unknown/debug/yew-app.wasm`にあるはずです。
もしクレートに何か別の名前をつけた場合、Wasmファイルの名前は`yew-app.wasm`ではなく、`Cargo.toml`ファイルに
`package.name`として名前をつけたものになるでしょう。

```bash
cargo build --target wasm32-unknown-unknown
```

次に、wasm-bindgenのCLIを動かしましょう。
このコマンドは`--out-dir`のディレクトリにいくつかのファイルを生成し、その中にはWasmバイナリを読み込んで動かすための
コンパイルされたWebAssemblyとJavaScriptのラッパーが入っています。
現在のブラウザは直接WebAssemblyファイルを読み込むことができないため、代わりにJavaScript経由で読み込まれるなければならず、
そのためにこれらのラッパーが必要となります。
[サンプルアプリを作る(../build-a-sample-app.md)の例では`static`フォルダにファイルが生成されるようにしており
(そのために`wasm-bindgen`へ`--out-dir static`と渡す必要があります)、
`wasm.js`と`wasm_bg.wasm`という名前になります(`wasm-bindgen`へ`--out-name wasm`と渡すことで実現できます)

```bash
wasm-bindgen --target web --out-dir static --out-name wasm target/wasm32-unknown-unknown/debug/appname.wasm --no-typescript
```

## アプリをサーブする

好きなサーバーを使ってください。
ここではシンプルなPythonのサーバーを使います。

```bash
python -m http.server 8000
```

## サポートされているターゲット

* `wasm32-unknown-unknown`

## 参考ドキュメント

* [The `wasm-bindgen` docs](https://rustwasm.github.io/docs/wasm-bindgen/)
