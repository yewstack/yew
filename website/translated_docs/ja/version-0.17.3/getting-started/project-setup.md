---
title: Introduction
description: Set yourself up for success
---

# 始める

## Rust

まずはじめにRustが必要です。Rustとビルドツールの`cargo`をインストールするために、以下の[公式サイト](https://www.rust-lang.org/tools/install)
を見てください。

## **Wasm ビルドツール**

WebAssemblyとJavaScriptの互換を持たせるために他にツールが必要です。さらに、選んだツールに応じてブラウザでアプリから`.wasm`ファイルを実行するのに
必要なJavaScriptラッパーのコードを生成し、これによってデプロイやパッケージ化での頭痛の種を軽減させるのに役立ちます。

### [**`wasm-pack`**](https://rustwasm.github.io/docs/wasm-pack/)

Rust / Wasm活動チームによって開発されているCLIツールで、WebAssemblyをパッケージ化することができます。
Webpackには[`wasm-pack-plugin`](https://github.com/wasm-tool/wasm-pack-plugin)が最もよく使われています。

[`wasm-pack`で始める](project-setup/using-wasm-pack.md)

### [**`wasm-bindgen`**](https://rustwasm.github.io/docs/wasm-bindgen/)

Rust/Wasm活動チームによって開発されているライブラリとCLIツールで、JS / WebAssemblyの互換性を持たせるための低レベルなツールです
(`wasm-pack`で内部的に使われています)。
`wasm-bindgen`は手書きのJavaScriptでWebAssemblyのバイナリを使う必要があるため、直接使うのは非推奨です。
しかし、詳細な情報については[**`wasm-bindgen` ガイド**](https://rustwasm.github.io/docs/wasm-bindgen/)から得られます。

[`wasm-bindgen`で始める。](project-setup/using-wasm-bindgen.md)

### [**`cargo-web`**](https://github.com/koute/cargo-web)

`wasm-pack`と`wasm-bindgen`を導入する前は好まれたWebワークフローツールです。
`wasm-pack`がサポートされていないサンプルを動かすのにインストールする価値があり、依然として**最もお手軽に**始められる方法です。

[`cargo web`で始める](project-setup/using-cargo-web.md)

### 比較

<table>
  <thead>
    <tr>
      <th style="text-align:left"></th>
      <th style="text-align:left"><code>wasm-pack</code>
      </th>
      <th style="text-align:left"><code>wasm-bindgen</code>
      </th>
      <th style="text-align:left"><code>cargo-web</code>
      </th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td style="text-align:left">プロジェクトの進行状況</td>
      <td style="text-align:left"><a href="https://rustwasm.github.io/">Rust / Wasm活動チーム</a>により活発にメンテナンス
      </td>
      <td style="text-align:left"><a href="https://rustwasm.github.io/">Rust / Wasm 活動チーム</a>により活発にメンテナンス
      </td>
      <td style="text-align:left">6ヶ月間GitHubでの活発な活動無し</td>
    </tr>
    <tr>
      <td style="text-align:left">開発体験</td>
      <td style="text-align:left">ほぼ大丈夫! <code>webpack</code>があればなお良い。</td>
      <td
      style="text-align:left">だいたい大丈夫。開発においては少し流れを書かないといけない。</td>
        <td style="text-align:left">しっかり動く！完結していて、外部ライブラリに頼る必要無し。</td>
    </tr>
    <tr>
      <td style="text-align:left">ローカルサーバー</td>
      <td style="text-align:left"><code>webpack</code>プラグインによるサポートあり</td>
      <td style="text-align:left">サポート無し</td>
      <td style="text-align:left">サポートあり</td>
    </tr>
    <tr>
      <td style="text-align:left">ローカル環境での変更による自動再ビルド</td>
      <td style="text-align:left"><code>webpack</code>プラグインによるサポートあり</td>
      <td style="text-align:left">サポート無し</td>
      <td style="text-align:left">サポートあり</td>
    </tr>
    <tr>
      <td style="text-align:left">ヘッドレスブラウザテスト</td>
      <td style="text-align:left"><a href="https://rustwasm.github.io/docs/wasm-pack/commands/test.html">サポートあり</a>
      </td>
      <td style="text-align:left"><a href="https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html">サポートあり</a>
      </td>
      <td style="text-align:left"><a href="https://github.com/koute/cargo-web#features">サポートあり</a>
      </td>
    </tr>
    <tr>
      <td style="text-align:left">サポートされているターゲット</td>
      <td style="text-align:left">
        <ul>
          <li><code>wasm32-unknown-unknown</code>
          </li>
        </ul>
      </td>
      <td style="text-align:left">
        <ul>
          <li><code>wasm32-unknown-unknown</code>
          </li>
        </ul>
      </td>
      <td style="text-align:left">
        <ul>
          <li><code>wasm32-unknown-unknown</code>
          </li>
          <li><code>wasm32-unknown-emscripten</code>
          </li>
          <li><code>asmjs-unknown-emscripten</code>
          </li>
        </ul>
      </td>
    </tr>
    <tr>
      <td style="text-align:left"><code>web-sys</code></td>
      <td style="text-align:left">互換性あり</td>
      <td style="text-align:left">互換性あり</td>
      <td style="text-align:left">互換性無し</td>
    </tr>
    <tr>
      <td style="text-align:left"><code>stdweb</code></td>
      <td style="text-align:left">互換性あり</td>
      <td style="text-align:left">互換性あり</td>
      <td style="text-align:left">互換性あり</td>
    </tr>
    <tr>
      <td style="text-align:left">使用例</td>
      <td style="text-align:left"><a href="https://github.com/yewstack/yew-wasm-pack-minimal">入門用テンプレート</a>
      </td>
      <td style="text-align:left">
        Yewで<a href="https://github.com/yewstack/yew/blob/master/examples/build.sh">作る例</a>
      </td>
      <td style="text-align:left">
        Yewで<a href="https://www.github.com/yewstack/yew/tree/master/packages/yew-stdweb/examples">作る例</a>
      </td>
    </tr>
  </tbody>
</table>

