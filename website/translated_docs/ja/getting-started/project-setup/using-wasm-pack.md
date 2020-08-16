---
title: Using wasm-pack
---

このツールはRust / Wasm活動チームによって開発され、WebAssemblyのアプリを作るのに使われれるツールで最も活発に開発されているものです。
コードを`npm`モジュールへパッケージ化するのをサポートし、既存のJavaScriptのアプリと簡単に統合できる
[Webpack plugin](https://github.com/wasm-tool/wasm-pack-plugin)がついています。
詳しい情報は[the `wasm-pack` documentation](https://rustwasm.github.io/docs/wasm-pack/introduction.html)にあります。

:::注意
`wasm-pack`を使う際は`Cargo.toml`のcrate-typeは`cdylib`である必要があります。
:::

## インストール

```bash
cargo install wasm-pack
```

## ビルド

このコマンドはJavaScriptラッパーとアプリのWebAssemblyをまとめたものを`./pkg`ディレクトリに生成し、アプリをスタートすることができます。
This command will produce a bundle in the `./pkg` directory with your app's compiled WebAssembly 
along with a JavaScript wrapper which can be used to start your application.

```bash
wasm-pack build --target web
```

## バンドル

ロールアップにについては詳しくは[ガイド](https://rollupjs.org/guide/en/#quick-start)をご覧ください。

```bash
rollup ./main.js --format iife --file ./pkg/bundle.js
```

## サーブ

好きなサーバーを使ってください。
ここではシンプルなPythonのサーバーを使ってアプリをサーブします。

```bash
python -m http.server 8000
```

## サポートされているターゲット

* `wasm32-unknown-unknown`
