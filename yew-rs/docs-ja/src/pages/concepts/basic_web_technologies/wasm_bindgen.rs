pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![
            link("https://github.com/rustwasm/wasm-bindgen", vec![
                code("wasm-bindgen"),
            ]),
            text(" は、JavaScript と Rust 関数の間に呼び出しブリッジを作成するためのライブラリおよびツールです。これは "),
            link("https://rustwasm.github.io/", vec![text("Rust と WebAssembly ワーキンググループ")]),
            text(" によって Rust で構築されました。"),
        ]),
        p(vec![
            text("Yew は "),
            code("wasm-bindgen"),
            text(" を使用して、いくつかのクレートを介してブラウザと対話します："),
        ]),
        ul(vec![
            li(vec![link("https://crates.io/crates/js-sys", vec![code("js-sys")])]),
            li(vec![link("https://crates.io/crates/wasm-bindgen", vec![code("wasm-bindgen")])]),
            li(vec![link("https://crates.io/crates/wasm-bindgen-futures", vec![code("wasm-bindgen-futures")])]),
            li(vec![link("https://crates.io/crates/web-sys", vec![code("web-sys")])]),
        ]),
        p(vec![
            text("このセクションでは、これらのクレートをより抽象的なレベルから探求し、Yew での "),
            code("wasm-bindgen"),
            text(" API の理解と使用を容易にします。"),
            code("wasm-bindgen"),
            text(" および関連するクレートに関する詳細なガイドについては、"),
            link("https://wasm-bindgen.github.io/wasm-bindgen/", vec![text("wasm-bindgen ガイド")]),
            text(" を参照してください。"),
        ]),
        p(vec![
            text("上記のクレートのドキュメントについては、"),
            link("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html", vec![
                text("wasm-bindgen docs.rs"),
            ]),
            text(" を参照してください。"),
        ]),
        admonition(AdmonitionType::Tip, None, vec![
            p(vec![
                code("wasm-bindgen"),
                text(" doc.rs 検索を使用して、"),
                code("wasm-bindgen"),
                text(" を使用してインポートされたブラウザ API および JavaScript タイプを見つけます。"),
            ]),
        ]),
        h2(vec![
            link("https://crates.io/crates/wasm-bindgen", vec![text("wasm-bindgen")]),
        ]),
        p(vec![
            text("このクレートは、上記の他のクレートに多くの構成要素を提供します。このセクションでは、"),
            code("wasm-bindgen"),
            text(" クレートの主要な領域の 2 つ、つまりマクロと、何度も目にするタイプ/トレイトのいくつかについてのみ説明します。"),
        ]),
        h3(vec![text("#[wasm_bindgen] マクロ")]),
        p(vec![
            code("#[wasm_bindgen]"),
            text(" マクロは Rust と JavaScript の間のインターフェースを提供し、両者の間で変換を行うシステムを提供します。このマクロの使用はより高度であり、外部の JavaScript ライブラリを使用する場合を除いて使用しないでください。"),
            code("js-sys"),
            text(" および "),
            code("web-sys"),
            text(" クレートは、組み込みの JavaScript タイプおよびブラウザ API に対して "),
            code("wasm-bindgen"),
            text(" 定義を提供します。"),
        ]),
        p(vec![
            code("#[wasm-bindgen]"),
            text(" マクロを使用して、特定のバージョンの "),
            link("https://developer.mozilla.org/en-US/docs/Web/API/Console/log", vec![
                code("console.log"),
            ]),
            text(" 関数をインポートする簡単な例を見てみましょう。"),
        ]),
        code_block("rust", r#"use wasm_bindgen::prelude::*;

// まず、`web_sys` を使用せずに `console.log` を手動でバインドしてみましょう。
// ここでは、手動で `#[wasm_bindgen]` アノテーションを書きます。プログラムの正確性はこれらのアノテーションの正確性に依存します！
#[wasm_bindgen]
extern "C" {
    // ここで `js_namespace` を使用して `console.log(..)` をバインドします。`log(..)` だけではありません。
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // `console.log` は多態的なので、複数のシグネチャを使用してバインドできます。
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // 複数の引数も可能です！
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// インポートされた関数を使用します！
log("Hello from Rust!");
log_u32(42);
log_many("Logging", "many values!");"#),
        p(vec![
            italic(vec![
                text("この例は、"),
                link("https://wasm-bindgen.github.io/wasm-bindgen/examples/console-log.html", vec![
                    text("1.2 wasm-bindgen ガイドの console.log を使用する"),
                ]),
                text(" に基づいています。"),
            ]),
        ]),
        h3_id("simulating-inheritance", vec![text("継承のシミュレーション")]),
        p(vec![
            text("JavaScript クラス間の継承は、JavaScript 言語のコア機能であり、DOM（ドキュメントオブジェクトモデル）はそれを中心に設計されています。"),
            code("wasm-bindgen"),
            text(" を使用して型をインポートする際にも、それらの継承関係を記述する属性を追加できます。"),
        ]),
        p(vec![
            text("Rust では、この継承関係は "),
            link("https://doc.rust-lang.org/std/ops/trait.Deref.html", vec![code("Deref")]),
            text(" と "),
            link("https://doc.rust-lang.org/std/convert/trait.AsRef.html", vec![code("AsRef")]),
            text(" トレイトを使用して表現されます。ここで例を挙げると役立つかもしれません。例えば、"),
            code("A"),
            text("、"),
            code("B"),
            text("、"),
            code("C"),
            text(" という 3 つの型があり、"),
            code("C"),
            text(" が "),
            code("B"),
            text(" を拡張し、"),
            code("B"),
            text(" が "),
            code("A"),
            text(" を拡張しているとします。"),
        ]),
        p(vec![
            text("これらの型をインポートする際、"),
            code("#[wasm-bindgen]"),
            text(" マクロは次のように "),
            code("Deref"),
            text(" と "),
            code("AsRef"),
            text(" トレイトを実装します："),
        ]),
        ul(vec![
            li(vec![code("C"), text(" は "), code("B"), text(" に "), code("Deref"), text(" できます")]),
            li(vec![code("B"), text(" は "), code("A"), text(" に "), code("Deref"), text(" できます")]),
            li(vec![code("C"), text(" は "), code("B"), text(" に "), code("AsRef"), text(" できます")]),
            li(vec![code("C"), text(" と "), code("B"), text(" はどちらも "), code("A"), text(" に "), code("AsRef"), text(" できます")]),
        ]),
        p(vec![
            text("これらの実装により、"),
            code("C"),
            text(" のインスタンスで "),
            code("A"),
            text(" のメソッドを呼び出したり、"),
            code("C"),
            text(" を "),
            code("&B"),
            text(" または "),
            code("&A"),
            text(" として使用したりできます。"),
        ]),
        p(vec![
            text("注意すべき点は、"),
            code("#[wasm-bindgen]"),
            text(" を使用してインポートされたすべての型には同じルート型があり、それを上記の例の "),
            code("A"),
            text(" と見なすことができるということです。この型は "),
            link("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html", vec![
                code("JsValue"),
            ]),
            text(" であり、以下にそのセクションがあります。"),
        ]),
        p(vec![
            italic(vec![
                link("https://wasm-bindgen.github.io/wasm-bindgen/reference/attributes/on-js-imports/extends.html", vec![
                    text("wasm-bindgen ガイドの extends セクション"),
                ]),
            ]),
        ]),
        h3_id("jsvalue", vec![
            link("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html", vec![
                text("JsValue"),
            ]),
        ]),
        p(vec![
            text("これは JavaScript が所有するオブジェクトの表現であり、"),
            code("wasm-bindgen"),
            text(" のルートキャプチャ型です。"),
            code("wasm-bindgen"),
            text(" からの任意の型は "),
            code("JsValue"),
            text(" です。これは、JavaScript には強い型システムがないため、変数 "),
            code("x"),
            text(" を受け取る任意の関数がその型を定義しないため、"),
            code("x"),
            text(" は有効な JavaScript 値である可能性があるためです。したがって "),
            code("JsValue"),
            text(" です。"),
            code("JsValue"),
            text(" を受け取るインポート関数や型を使用している場合、技術的には任意のインポート値が有効です。"),
        ]),
        p(vec![
            code("JsValue"),
            text(" は関数で受け取ることができますが、その関数は特定の型のみを受け取る可能性があり、それがパニックを引き起こす可能性があります。したがって、元の "),
            code("wasm-bindgen"),
            text(" API を使用する場合は、インポートされた JavaScript のドキュメントを確認して、その値が特定の型でない場合に例外（パニック）を引き起こすかどうかを確認してください。"),
        ]),
        p(vec![
            italic(vec![
                link("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html", vec![
                    text("JsValue ドキュメント"),
                ]),
                text("。"),
            ]),
        ]),
        h3_id("jscast", vec![
            link("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html", vec![
                text("JsCast"),
            ]),
        ]),
        p(vec![
            text("Rust には強い型システムがありますが、JavaScript にはありません😞。Rust がこれらの強い型を維持しながらも便利であるために、WebAssembly ワーキンググループは非常に巧妙な機能 "),
            code("JsCast"),
            text(" を提案しました。これは、ある JavaScript \"型\" から別の \"型\" への変換を支援するものです。これは曖昧に聞こえますが、ある型が別の型であることがわかっている場合、"),
            code("JsCast"),
            text(" の関数を使用してある型から別の型にジャンプできます。"),
            code("web-sys"),
            text("、"),
            code("wasm_bindgen"),
            text("、"),
            code("js-sys"),
            text(" を使用する際にこの機能を理解しておくと便利です。これらのクレートから多くの型が "),
            code("JsCast"),
            text(" を実装していることに気付くでしょう。"),
        ]),
        p(vec![
            code("JsCast"),
            text(" はチェック付きとチェックなしの変換メソッドを提供します。したがって、実行時にオブジェクトがどの型であるかわからない場合は、変換を試みることができ、失敗する可能性のある型として "),
            link("https://doc.rust-lang.org/std/option/enum.Option.html", vec![code("Option")]),
            text(" や "),
            link("https://doc.rust-lang.org/std/result/enum.Result.html", vec![code("Result")]),
            text(" を返します。"),
        ]),
        p(vec![
            text("一般的な例は "),
            link("/ja/docs/concepts/basic-web-technologies/web-sys", vec![code("web-sys")]),
            text(" で、イベントのターゲットを取得しようとする場合です。ターゲット要素が何であるかを知っているかもしれませんが、"),
            link("https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html", vec![
                code("web_sys::Event"),
            ]),
            text(" API は常に "),
            link("https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.target", vec![
                code("Option<web_sys::EventTarget>"),
            ]),
            text(" を返します。その要素型に変換する必要があり、そのメソッドを呼び出すことができます。"),
        ]),
        code_block("rust", "// このトレイトを最初にインポートする必要があります
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, HtmlInputElement, HtmlSelectElement};

fn handle_event(event: Event) {
    let target: EventTarget = event
        .target()
        .expect(\"I'm sure this event has a target!\");

    // もしかしたらターゲットは選択要素かもしれませんか？
    if let Some(select_element) = target.dyn_ref::<HtmlSelectElement>() {
        // 別のことをする
        return;
    }

    // それが選択要素でないことが確実であれば、入力要素であることが確実です！
    let input_element: HtmlInputElement = target.unchecked_into();
}"),
        p(vec![
            link("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_ref", vec![
                code("dyn_ref"),
            ]),
            text(" メソッドはチェック付きの変換であり、"),
            code("Option<&T>"),
            text(" を返します。これは、変換が失敗した場合に元の型を再度使用できることを意味し、"),
            code("None"),
            text(" を返します。"),
            link("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into", vec![
                code("dyn_into"),
            ]),
            text(" メソッドは "),
            code("self"),
            text(" を消費し、Rust の "),
            code("into"),
            text(" メソッドの規約に従い、"),
            code("Result<T, Self>"),
            text(" 型を返します。変換が失敗した場合、元の "),
            code("Self"),
            text(" 値は "),
            code("Err"),
            text(" に返されます。再試行するか、元の型で他の操作を行うことができます。"),
        ]),
        p(vec![
            italic(vec![
                link("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html", vec![
                    text("JsCast ドキュメント"),
                ]),
                text("。"),
            ]),
        ]),
        h3(vec![
            link("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html", vec![
                text("Closure"),
            ]),
        ]),
        p(vec![
            code("Closure"),
            text(" 型は、Rust のクロージャを JavaScript に渡す方法を提供します。健全性の理由から、JavaScript に渡されるクロージャは "),
            code("'static"),
            text(" ライフタイムを持つ必要があります。"),
        ]),
        p(vec![
            text("この型は「ハンドル」であり、破棄されると、それが参照する JS クロージャを無効にします。"),
            code("Closure"),
            text(" が破棄された後、JS 内のクロージャの使用はすべて例外を引き起こします。"),
        ]),
        p(vec![
            code("Closure"),
            text(" は、"),
            link("https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/struct.Function.html", vec![
                code("&js_sys::Function"),
            ]),
            text(" 型を受け取る "),
            code("js-sys"),
            text(" または "),
            code("web-sys"),
            text(" API を使用する際に一般的に使用されます。Yew で "),
            code("Closure"),
            text(" を使用する例は、"),
            link("/ja/docs/concepts/html/events#using-closure-verbose", vec![text("Using Closure セクション")]),
            text(" の "),
            link("/ja/docs/concepts/html/events", vec![text("Events")]),
            text(" ページにあります。"),
        ]),
        p(vec![
            italic(vec![
                link("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html", vec![
                    text("Closure ドキュメント"),
                ]),
                text("。"),
            ]),
        ]),
        h2(vec![
            link("https://crates.io/crates/js-sys", vec![text("js-sys")]),
        ]),
        p(vec![
            code("js-sys"),
            text(" クレートは、JavaScript の標準組み込みオブジェクトのバインディング/インポートを提供します。これには、それらのメソッドやプロパティが含まれます。"),
        ]),
        p(vec![
            text("これは Web API を含みません。Web API は "),
            link("/ja/docs/concepts/basic-web-technologies/web-sys", vec![code("web-sys")]),
            text(" の役割です！"),
        ]),
        p(vec![
            italic(vec![
                link("https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/index.html", vec![
                    text("js-sys ドキュメント"),
                ]),
                text("。"),
            ]),
        ]),
        h2(vec![
            link("https://crates.io/crates/wasm-bindgen-futures", vec![text("wasm-bindgen-futures")]),
        ]),
        p(vec![
            code("wasm-bindgen-futures"),
            text(" クレートは、JavaScript の Promise 型を Rust の "),
            link("https://doc.rust-lang.org/stable/std/future/trait.Future.html", vec![code("Future")]),
            text(" として扱うためのブリッジを提供し、Rust の Future を JavaScript の Promise に変換するユーティリティを含みます。Rust（wasm）で非同期または他のブロッキング作業を処理する際に役立ち、JavaScript のイベントや JavaScript I/O プリミティブと対話する能力を提供します。"),
        ]),
        p(vec![text("現在、このクレートには3つの主要なインターフェースがあります：")]),
        ol(vec![
            li_blocks(vec![
                p(vec![
                    link("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/struct.JsFuture.html", vec![
                        code("JsFuture"),
                    ]),
                    text(" - "),
                    link("https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/struct.Promise.html", vec![
                        code("Promise"),
                    ]),
                    text(" を使用して構築された型で、"),
                    code("Future<Output=Result<JsValue, JsValue>>"),
                    text(" として使用できます。"),
                    code("Promise"),
                    text(" が解決されると、この "),
                    code("Future"),
                    text(" は "),
                    code("Ok"),
                    text(" に解決され、"),
                    code("Promise"),
                    text(" が拒否されると "),
                    code("Err"),
                    text(" に解決され、それぞれ "),
                    code("Promise"),
                    text(" の解決または拒否の値を含みます。"),
                ]),
            ]),
            li_blocks(vec![
                p(vec![
                    link("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.future_to_promise.html", vec![
                        code("future_to_promise"),
                    ]),
                    text(" - Rust の "),
                    code("Future<Output=Result<JsValue, JsValue>>"),
                    text(" を JavaScript の "),
                    code("Promise"),
                    text(" に変換します。Future の結果は、JavaScript 内の解決または拒否された "),
                    code("Promise"),
                    text(" に変換されます。"),
                ]),
            ]),
            li_blocks(vec![
                p(vec![
                    link("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html", vec![
                        code("spawn_local"),
                    ]),
                    text(" - 現在のスレッドで "),
                    code("Future<Output = ()>"),
                    text(" を生成します。これは、Rust 内で Future を実行する最良の方法であり、JavaScript に送信するのではなく、Rust 内で実行します。"),
                ]),
            ]),
        ]),
        p(vec![
            italic(vec![
                link("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/index.html", vec![
                    text("wasm-bindgen-futures ドキュメント"),
                ]),
                text("。"),
            ]),
        ]),
        h3(vec![
            link("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html", vec![
                text("spawn_local"),
            ]),
        ]),
        p(vec![
            code("spawn_local"),
            text(" は、非同期 API を使用するライブラリを使用する際に、Yew で "),
            code("wasm-bindgen-futures"),
            text(" クレートの最も一般的に使用される部分です。"),
        ]),
        code_block("rust", "use web_sys::console;
use wasm_bindgen_futures::spawn_local;

async fn my_async_fn() -> String { String::from(\"Hello\") }

spawn_local(async {
    let mut string = my_async_fn().await;
    string.push_str(\", world!\");
    // \"Hello, world!\" を出力します
    console::log_1(&string.into());
});"),
        p(vec![
            text("Yew はいくつかの API に futures のサポートを追加しており、特に "),
            code("async"),
            text(" ブロックを受け入れる "),
            code("callback_future"),
            text(" を作成できることが注目されます。これは内部的に "),
            code("spawn_local"),
            text(" を使用しています。"),
        ]),
        p(vec![
            italic(vec![
                link("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html", vec![
                    text("spawn_local ドキュメント"),
                ]),
                text("。"),
            ]),
        ]),
    ])
}

crate::doc_page!(
    "wasm-bindgen",
    "/ja/docs/concepts/basic-web-technologies/wasm-bindgen",
    page_content()
);
