---
title: Introduction
---

## Yewとは?

**Yew** は [WebAssembly](https://webassembly.org/) によってマルチスレッドなWebアプリのフロントエンドを作ることができる、モダンな [Rust](https://www.rust-lang.org/) のフレームワークです。

* インタラクティブなUIを簡単に作ることができる、**コンポーネントベース** のフレームワークです. [React](https://reactjs.org/) や [Elm](https://elm-lang.org/) のようなフレームワークを使用したことがある開発者はYewを違和感なく使うことができるでしょう。
* DOMのAPIを呼び出すのを最小化し、開発者がWebワーカーによって簡単に処理を軽量化できることで **素晴らしいパフォーマンス** を発揮します。
* **JavaScriptとの互換性** をサポートし、開発者はnpmのパッケージを活用し既存のJavaScriptアプリと統合させることができます。

### コミュニティに参加する 😊

* [GitHub issues page](https://github.com/yewstack/yew/issues) でバグを報告したり機能について議論できます。
* プルリクエストはぜひウェルカムです。ご協力いただけるなら [good first issues](https://github.com/yewstack/yew/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) をご確認ください。
* 私たちの [Discord chat](https://discord.gg/VQck8X4) はとても活発で質問するのに良い場所です。

### 始める準備はいいですか?

以下のリンクをクリックして初めてのYewアプリの作り方を学び、コミュニティのプロジェクト例を見てみましょう。

[始める](getting-started/project-setup.md)

### まだ満足していませんか?

このプロジェクトは先進的な技術によって成り立っており、これからの基礎となるプロジェクトを作っていきたい開発者にとっては素晴らしいものです。
ここではYewのようなフレームワークがどうしてWeb開発における未来となると考えられるかについての理由を述べていきます。

#### 待って、どうしてWebAssembly?

WebAssembly _\(Wasm\)_ はRustがコンパイル可能な軽量で低レベルな言語です。
WebAssemblyはブラウザでネイティブ並の速度で動き、JavaScriptと互換性があり、そして全ての主要なブラウザでサポートされています。
アプリでWebAssemblyをどう使いこなすかについては [use cases](https://webassembly.org/docs/use-cases/) のリストをご確認ください。

気をつけなければいけないこととして、Wasmは\(まだ\)Webアプリのパフォーマンスを改善する銀の弾丸ではありません。
現時点では、DOMのAPIをWebAssemblyから使うのはまだJavaScriptから直接呼ぶよりも遅いのです。
これは [WebAssembly Interface Types](https://github.com/WebAssembly/interface-types/blob/master/proposals/interface-types/Explainer.md) が解決しようとしている今のところの課題です。
詳しい情報についてはMozillaの提案が書いてある [excellent article](https://hacks.mozilla.org/2019/08/webassembly-interface-types/)
をご確認ください。

#### わかった、でもどうしてRust?

Rustは目まぐるしいほど高速に動作し、素晴らしい型と所有権モデルによって信頼性があります。
Rustには厳しい学習曲線がありますが、努力するだけの十分な価値があります。
RustはStack OverFlowによる開発者調査で5年連続で最も愛されている言語に選ばれています:
[2016](https://insights.stackoverflow.com/survey/2016#technology-most-loved-dreaded-and-wanted), 
[2017](https://insights.stackoverflow.com/survey/2017#most-loved-dreaded-and-wanted), 
[2018](https://insights.stackoverflow.com/survey/2018#technology-_-most-loved-dreaded-and-wanted-languages), 
[2019](https://insights.stackoverflow.com/survey/2019#technology-_-most-loved-dreaded-and-wanted-languages), 
[2020](https://insights.stackoverflow.com/survey/2020#most-loved-dreaded-and-wanted).

同時にRustは素晴らしい型システムと所有権モデルによって開発者がより安全なコードを書くようサポートしてくれます。
JavaScriptでの追跡するのが難しいバグよ、さよなら！
実際にRustではコードを走らせるまでもなくコンパイラによってバグのほとんどは捕捉されます。
そして心配することなかれ、アプリを走らせてエラーになったとしてもブラウザのコンソールでRustのコードを完全に追跡することができます。

#### 代わりは?

私たちは喜んで他のプロジェクトのアイデアを共有し、このワクワクする新たな技術のあらゆる可能性を実現できるよう皆が互いに助け合えると信じています。
もしYewがグッとこない場合は以下のプロジェクトがいいかもしれません。

* [Percy](https://github.com/chinedufn/percy) - _"RustとWebAssemblyで同一なWebアプリを作る組み立てツール"_
* [Seed](https://github.com/seed-rs/seed) - _"Webアプリを作るためのRustフレームワーク"_
