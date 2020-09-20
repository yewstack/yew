---
title: Optimizations
description: Make your app faster
---

# 最適化とベストプラクティス

## neq\_assign

親コンポーネントからpropsを受け取った際、`change`メソッドが呼ばれます。
これはコンポーネントの状態を更新することができるのに加え、コンポーネントがpropsが変わった際に再レンダリングするかどうかを決める
`ShouldRender`という真偽値を返すことができます。

再レンダリングはコストがかかるもので、もし避けられるのであれば避けるべきです。
一般的なルールとしてpropsが実際に変化した際にのみ再レンダリングすれば良いでしょう。
以下のコードブロックはこのルールを表しており、propsが前と変わったときに`true`を返します。

```rust
use yew::ShouldRender;

#[derive(PartialEq)]
struct ExampleProps;

struct Example {
    props: ExampleProps,
};

impl Example {
    fn change(&mut self, props: ExampleProps) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }
}
```

しかし我々は先に進んでいけます！
この6行のボイラープレードは`PartialEq`を実装したものにトレイトとブランケットを用いることで1行のコードへと落とし込むことができます。
[こちら](https://docs.rs/yewtil/*/yewtil/trait.NeqAssign.html)にて`yewtil`クレートの`NewAssign`トレイトを見てみてください。

## 効果的にスマートポインタを使う

**注意: このセクションで使われている用語がわからなければRust bookは
[スマートポインタについての章](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
があり、非常に有用です。** 

再レンダリングの際にpropsを作るデータを大量にコピーしないために、スマートポインタを用いてデータ自体ではなくデータへの参照だけを
コピーできます。
propsや子コンポーネントで関連するデータに実データではなく参照を渡すと、子コンポーネントでデータを変更する必要がなければ
データのコピーを避けることができます。
その際、`Rc::make_mut`によって変更したいデータの変更可能な参照を得ることができます。

これにより、propsが変更されたときにコンポーネントが再レンダリングされるかどうかを決めるかで`Component::change`に更なる恩恵があります。
なぜなら、データの値を比較する代わりに元々のポインタのアドレス (つまりデータが保管されている機械のメモリの場所) を比較できるためです。
2つのポインターが同じデータを指す場合、それらのデータの値は同じでなければならないのです。
ただし、その逆は必ずしも成り立たないことに注意してください!
もし2つのポインタが異なるのであれば、そのデータは同じである可能性があります。
この場合はデータを比較するべきでしょう。

この比較においては`PartialEq`ではなく`Rc::ptr_eq`を使う必要があります。
`PartialEq`は等価演算子`==`を使う際に自動的に使われます。
Rustのドキュメントには[`Rc::ptr_eq`についてより詳しく書いてあります](https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#method.ptr_eq)。

この最適化は`Copy`を実装していないデータの型に対して極めて有効なものです。
もしデータを簡単に変更できるのであれば、スマートポインタに取り換える必要はありません。
しかし`Vec`や`HashMap`、`String`などのような重たいデータの構造体に対してはスマートポインタを使うことで
パフォーマンスを改善することができるでしょう。

この最適化は値がまだ一度も子によって更新されていない場合に極めて有効で、親からほとんど更新されない場合においてもかなり有効です。
これにより、`Rc<_>s`が純粋なコンポーネントに対してプロパティの値をラップする良い一手となります。

## View関数

コードの可読性の理由から`html!`の部分を関数へと移植するのは意味があります。
これは、インデントを減らすのでコードを読みやすくするだけでなく、良いデザインパターンを産むことにも繋がるのです。
これらの関数は複数箇所で呼ばれて書くべきコード量を減らせるため、分解可能なアプリケーションを作ることができるのです。

## 純粋なコンポーネント

純粋なコンポーネントは状態を変化せず、ただ中身を表示してメッセージを普通の変更可能なコンポーネントへ渡すコンポーネントのことです。
View関数との違いとして、純粋なコンポーネントは式の構文\(`{some_view_function()}`\)ではなく
コンポーネントの構文\(`<SomePureComponent />`\)を使うことで`html!`マクロの中で呼ばれる点、
そして実装次第で記憶され (つまり、一度関数が呼ばれれば値は"保存"され、
同じ引数でもう一度呼ばれても値を再計算する必要がなく最初に関数が呼ばれたときの保存された値を返すことができる)、
先述の`neq_assign`ロジックを使う別々のpropsで再レンダリングを避けられる点があります。

Yewは純粋な関数やコンポーネントをサポートしていませんが、外部のクレートを用いることで実現できます。

## 関数型コンポーネント (a.k.a フック)

関数型コンポーネントはまだ開発中です！
開発状況については[プロジェクトボード](https://github.com/yewstack/yew/projects/3)に詳しく書いてあります。

## キー付きDOMノード

## ワークスペースでコンパイル時間を減らす

間違いなくYewを使う上での最大の欠点はコンパイルに時間がかかる点です。
プロジェクトのコンパイルにかかる時間は`html!`マクロに渡されるコードの量に関係しています。
これは小さなプロジェクトにはそこまで問題ないようですが、大きなアプリではコードを複数クレートに分割することでアプリに変更が加られた際に
コンパイラの作業量を減らすのが有効です。

一つ可能なやり方として、ルーティングとページ洗濯を担当するメインのクレートを作り、それぞれのページに対して別のクレートを作ることです。
そうして各ページは異なるコンポーネントか、`Html`を生成する大きな関数となります。
アプリの異なる部分を含むクレート同士で共有されるコードはプロジェクト全体で依存する分離したクレートに保存されます。
理想的には1回のコンパイルでコード全てを再ビルドせずメインのクレートかどれかのページのクレートを再ビルドするだけにすることです。
最悪なのは、"共通"のクレートを編集して、はじめに戻ってくることです: 
共有のクレートに依存している全てのコード、恐らく全てのコードをコンパイルすることです。

もしメインのクレートが重たすぎる、もしくは深くネストしたページ (例えば別ページのトップでレンダリングされるページ) 
で速く繰り返したい場合、クレートの例を用いてメインページの実装をシンプルにしたりトップで動かしているコンポーネントをレンダリングできます。

## バイナリサイズを小さくする

* Rustのコードを最適化する
  * `wee_alloc` \( tiny allocatorを使用 \)
  * `cargo.toml` \( release profileを定義 \)
* `wasm-opt`を用いてwasmのコードを最適化する

**注意: バイナリサイズを小さくするのについては[Rust Wasm Book](https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size)に詳しく書いてあります。**

### wee\_alloc

[wee\_alloc](https://github.com/rustwasm/wee_alloc)は小さなアロケーターで、Rustのバイナリで使用される通常のものより遥かに小さなものです。
デフォルトのアロケーターと置き換えることで、Wasmファイルをより小さくすることができ、速度とメモリのオーバーヘッドを軽減できます。

デフォルトのアロケータを含めないことによるサイズの増加と比較して、速度とメモリのオーバーヘッドが悪くなります。
ファイルサイズが小さいことで、ページの読み込みが速くなります。
そのため、アロケーションのタスクが非常に多い場合でなければデフォルトのものではなくtiny allocatorを利用することが一般的に推奨されています。

```rust
// `wee_alloc`を使用する。
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

### Cargo.toml

`Cargo.toml`で`[profile.release]`のセクションに設定を書き込むことでリリースビルドを小さくすることが可能です。

```text
[profile.release]
# バイナリに含むコードを少なくする
panic = 'abort' 
# コードベース全体での最適化 ( 良い最適化だがビルドが遅くなる)
codegen-units = 1
# サイズの最適化( よりアグレッシブに )
opt-level = 'z' 
# サイズの最適化
# opt-level = 's' 
# プログラム全体の分析によるリンク時最適化
lto = true
```

### wasm-opt

更に`wasm`のコードのサイズを最適化することができます。

The Rust Wasm BookにはWasmバイナリのサイズを小さくすることについてのセクションがあります:
[Shrinking .wasm size](https://rustwasm.github.io/book/game-of-life/code-size.html)

* `wasm-pack`でデフォルトの`wasm`のコードをリリースビルド時に最適化する
* `wasm-opt`によって直接`wasm`ファイルを最適化する

```text
wasm-opt wasm_bg.wasm -Os -o wasm_bg_opt.wasm
```

#### yew/examples/にある例を小さなサイズでビルドする

注意: `wasm-pack`はRustとWasmのコードへの最適化を組み合わせます。`wasm-bindgen`はこの例ではRustのサイズ最適化を用いていません。

| 使用したツール | サイズ |
| :--- | :--- |
| wasm-bindgen | 158KB |
| wasm-bindgen + wasm-opt -Os | 116KB |
| wasm-pack | 99 KB |

## 参考文献:
 * [The Rust Bookのスマートポインタに関する章](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
 * [the Rust Wasm Bookでのバイナリサイズを小さくすることについて](https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size)
 * [Rust profilesについてのドキュメント](https://doc.rust-lang.org/cargo/reference/profiles.html)
 * [binaryenプロジェクト](https://github.com/WebAssembly/binaryen)
