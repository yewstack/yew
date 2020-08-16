---
title: Debugging
---

# デバッグ

## パニック

Rustシンボルで良いスタックトレースをするには
[`console_error_panic`](https://github.com/rustwasm/console_error_panic_hook)クレートを使用してください。
注意として、`cargo-web`でビルドされたものとは互換性がありません。

## コンソールでのログ

一般的に、WasmのWebアプリはブラウザのAPIと連携することができ、`console.log`のAPIも例外ではありません。
いつくかの選択肢があります:

### [`wasm-logger`](https://crates.io/crates/wasm-logger)

このクレートはRustの`log`クレートと親和性があります。

```rust
// セットアップ
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
}

// 使用方法
log::info!("Update: {:?}", msg);
```

### [`ConsoleService`](https://docs.rs/yew/latest/yew/services/console/struct.ConsoleService.html)

このサービスはYewに含まれており、`"services"`の機能が有効化されている場合は利用可能です。

```rust
// 使用方法
ConsoleService::new()::info(format!("Update: {:?}", msg));
```

## ソースマップ

今のところはRust/WasmのWebアプリにはソースマップへの第一級のサポートがありません。
もちろん、これは変更される可能性があります。これが当てはまらない場合、または進捗が見られる場合は、変更を提案してください！

### 最新情報

\[2019年12月\] [Chrome DevTools update](https://developers.google.com/web/updates/2019/12/webassembly#the_future)

> やらなければいけないことがまだたくさんあります。例えばツール側ではEmscripten\(Binaryen\)とwasm-pack\(wasm-bindgen\)がそれらが実行する変換に関するDWARF情報の更新をまだサポートしていません。

\[2020\] [Rust Wasm デバッグガイド](https://rustwasm.github.io/book/reference/debugging.html#using-a-debugger)

> 残念なことに、WebAssemblyのデバッグの物語はまだ未成熟です。ほとんどのUnixのシステムでは[DWARF](http://dwarfstd.org/)は実行中のプログラムをソースレベルで検査するためにデバッガに必要な情報をエンコードするために使用されます。Windowsには同様の情報をエンコードする代替形式があります。現在、WebAssemblyに相当するものはありません。

\[2019\] [Rust Wasm ロードマップ](https://rustwasm.github.io/rfcs/007-2019-roadmap.html#debugging)

> デバッグはトリッキーです。なぜなら、多くの話はこの活動チームの手の届かないところにあり、WebAssembly の標準化団体とブラウザ開発者ツールを実装している人たちの両方に依存しているからです。
