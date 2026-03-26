pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            link!("https://github.com/rustwasm/wasm-bindgen",
                code("wasm-bindgen"),
            ),
            " は、JavaScript と Rust 関数の間に呼び出しブリッジを作成するためのライブラリおよびツールです。これは ",
            link!("https://rustwasm.github.io/", "Rust と WebAssembly ワーキンググループ"),
            " によって Rust で構築されました。",
        ],
        p![
            "Yew は ",
            code("wasm-bindgen"),
            " を使用して、いくつかのクレートを介してブラウザと対話します：",
        ],
        ul![
            li![link!("https://crates.io/crates/js-sys", code("js-sys"))],
            li![link!("https://crates.io/crates/wasm-bindgen", code("wasm-bindgen"))],
            li![link!("https://crates.io/crates/wasm-bindgen-futures", code("wasm-bindgen-futures"))],
            li![link!("https://crates.io/crates/web-sys", code("web-sys"))],
        ],
        p![
            "このセクションでは、これらのクレートをより抽象的なレベルから探求し、Yew での ",
            code("wasm-bindgen"),
            " API の理解と使用を容易にします。",
            code("wasm-bindgen"),
            " および関連するクレートに関する詳細なガイドについては、",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/", "wasm-bindgen ガイド"),
            " を参照してください。",
        ],
        p![
            "上記のクレートのドキュメントについては、",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html",
                "wasm-bindgen docs.rs",
            ),
            " を参照してください。",
        ],
        admonition!(AdmonitionType::Tip, None,
            p![
                code("wasm-bindgen"),
                " doc.rs 検索を使用して、",
                code("wasm-bindgen"),
                " を使用してインポートされたブラウザ API および JavaScript タイプを見つけます。",
            ],
        ),
        h2![
            link!("https://crates.io/crates/wasm-bindgen", "wasm-bindgen"),
        ],
        p![
            "このクレートは、上記の他のクレートに多くの構成要素を提供します。このセクションでは、",
            code("wasm-bindgen"),
            " クレートの主要な領域の 2 つ、つまりマクロと、何度も目にするタイプ/トレイトのいくつかについてのみ説明します。",
        ],
        h3!["#[wasm_bindgen] マクロ"],
        p![
            code("#[wasm_bindgen]"),
            " マクロは Rust と JavaScript の間のインターフェースを提供し、両者の間で変換を行うシステムを提供します。このマクロの使用はより高度であり、外部の JavaScript ライブラリを使用する場合を除いて使用しないでください。",
            code("js-sys"),
            " および ",
            code("web-sys"),
            " クレートは、組み込みの JavaScript タイプおよびブラウザ API に対して ",
            code("wasm-bindgen"),
            " 定義を提供します。",
        ],
        p![
            code("#[wasm-bindgen]"),
            " マクロを使用して、特定のバージョンの ",
            link!("https://developer.mozilla.org/en-US/docs/Web/API/Console/log",
                code("console.log"),
            ),
            " 関数をインポートする簡単な例を見てみましょう。",
        ],
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
        p![
            italic![
                "この例は、",
                link!("https://wasm-bindgen.github.io/wasm-bindgen/examples/console-log.html",
                    "1.2 wasm-bindgen ガイドの console.log を使用する",
                ),
                " に基づいています。",
            ],
        ],
        h3_id!("simulating-inheritance", "継承のシミュレーション"),
        p![
            "JavaScript クラス間の継承は、JavaScript 言語のコア機能であり、DOM（ドキュメントオブジェクトモデル）はそれを中心に設計されています。",
            code("wasm-bindgen"),
            " を使用して型をインポートする際にも、それらの継承関係を記述する属性を追加できます。",
        ],
        p![
            "Rust では、この継承関係は ",
            link!("https://doc.rust-lang.org/std/ops/trait.Deref.html", code("Deref")),
            " と ",
            link!("https://doc.rust-lang.org/std/convert/trait.AsRef.html", code("AsRef")),
            " トレイトを使用して表現されます。ここで例を挙げると役立つかもしれません。例えば、",
            code("A"),
            "、",
            code("B"),
            "、",
            code("C"),
            " という 3 つの型があり、",
            code("C"),
            " が ",
            code("B"),
            " を拡張し、",
            code("B"),
            " が ",
            code("A"),
            " を拡張しているとします。",
        ],
        p![
            "これらの型をインポートする際、",
            code("#[wasm-bindgen]"),
            " マクロは次のように ",
            code("Deref"),
            " と ",
            code("AsRef"),
            " トレイトを実装します：",
        ],
        ul![
            li![code("C"), " は ", code("B"), " に ", code("Deref"), " できます"],
            li![code("B"), " は ", code("A"), " に ", code("Deref"), " できます"],
            li![code("C"), " は ", code("B"), " に ", code("AsRef"), " できます"],
            li![code("C"), " と ", code("B"), " はどちらも ", code("A"), " に ", code("AsRef"), " できます"],
        ],
        p![
            "これらの実装により、",
            code("C"),
            " のインスタンスで ",
            code("A"),
            " のメソッドを呼び出したり、",
            code("C"),
            " を ",
            code("&B"),
            " または ",
            code("&A"),
            " として使用したりできます。",
        ],
        p![
            "注意すべき点は、",
            code("#[wasm-bindgen]"),
            " を使用してインポートされたすべての型には同じルート型があり、それを上記の例の ",
            code("A"),
            " と見なすことができるということです。この型は ",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html",
                code("JsValue"),
            ),
            " であり、以下にそのセクションがあります。",
        ],
        p![
            italic![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/reference/attributes/on-js-imports/extends.html",
                    "wasm-bindgen ガイドの extends セクション",
                ),
            ],
        ],
        h3_id!("jsvalue",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html",
                "JsValue",
            ),
        ),
        p![
            "これは JavaScript が所有するオブジェクトの表現であり、",
            code("wasm-bindgen"),
            " のルートキャプチャ型です。",
            code("wasm-bindgen"),
            " からの任意の型は ",
            code("JsValue"),
            " です。これは、JavaScript には強い型システムがないため、変数 ",
            code("x"),
            " を受け取る任意の関数がその型を定義しないため、",
            code("x"),
            " は有効な JavaScript 値である可能性があるためです。したがって ",
            code("JsValue"),
            " です。",
            code("JsValue"),
            " を受け取るインポート関数や型を使用している場合、技術的には任意のインポート値が有効です。",
        ],
        p![
            code("JsValue"),
            " は関数で受け取ることができますが、その関数は特定の型のみを受け取る可能性があり、それがパニックを引き起こす可能性があります。したがって、元の ",
            code("wasm-bindgen"),
            " API を使用する場合は、インポートされた JavaScript のドキュメントを確認して、その値が特定の型でない場合に例外（パニック）を引き起こすかどうかを確認してください。",
        ],
        p![
            italic![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html",
                    "JsValue ドキュメント",
                ),
                "。",
            ],
        ],
        h3_id!("jscast",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html",
                "JsCast",
            ),
        ),
        p![
            "Rust には強い型システムがありますが、JavaScript にはありません😞。Rust がこれらの強い型を維持しながらも便利であるために、WebAssembly ワーキンググループは非常に巧妙な機能 ",
            code("JsCast"),
            " を提案しました。これは、ある JavaScript \"型\" から別の \"型\" への変換を支援するものです。これは曖昧に聞こえますが、ある型が別の型であることがわかっている場合、",
            code("JsCast"),
            " の関数を使用してある型から別の型にジャンプできます。",
            code("web-sys"),
            "、",
            code("wasm_bindgen"),
            "、",
            code("js-sys"),
            " を使用する際にこの機能を理解しておくと便利です。これらのクレートから多くの型が ",
            code("JsCast"),
            " を実装していることに気付くでしょう。",
        ],
        p![
            code("JsCast"),
            " はチェック付きとチェックなしの変換メソッドを提供します。したがって、実行時にオブジェクトがどの型であるかわからない場合は、変換を試みることができ、失敗する可能性のある型として ",
            link!("https://doc.rust-lang.org/std/option/enum.Option.html", code("Option")),
            " や ",
            link!("https://doc.rust-lang.org/std/result/enum.Result.html", code("Result")),
            " を返します。",
        ],
        p![
            "一般的な例は ",
            doc_link!(crate::pages::concepts::basic_web_technologies::web_sys, code("web-sys")),
            " で、イベントのターゲットを取得しようとする場合です。ターゲット要素が何であるかを知っているかもしれませんが、",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html",
                code("web_sys::Event"),
            ),
            " API は常に ",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.target",
                code("Option<web_sys::EventTarget>"),
            ),
            " を返します。その要素型に変換する必要があり、そのメソッドを呼び出すことができます。",
        ],
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
        p![
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_ref",
                code("dyn_ref"),
            ),
            " メソッドはチェック付きの変換であり、",
            code("Option<&T>"),
            " を返します。これは、変換が失敗した場合に元の型を再度使用できることを意味し、",
            code("None"),
            " を返します。",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into",
                code("dyn_into"),
            ),
            " メソッドは ",
            code("self"),
            " を消費し、Rust の ",
            code("into"),
            " メソッドの規約に従い、",
            code("Result<T, Self>"),
            " 型を返します。変換が失敗した場合、元の ",
            code("Self"),
            " 値は ",
            code("Err"),
            " に返されます。再試行するか、元の型で他の操作を行うことができます。",
        ],
        p![
            italic![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html",
                    "JsCast ドキュメント",
                ),
                "。",
            ],
        ],
        h3![
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html",
                "Closure",
            ),
        ],
        p![
            code("Closure"),
            " 型は、Rust のクロージャを JavaScript に渡す方法を提供します。健全性の理由から、JavaScript に渡されるクロージャは ",
            code("'static"),
            " ライフタイムを持つ必要があります。",
        ],
        p![
            "この型は「ハンドル」であり、破棄されると、それが参照する JS クロージャを無効にします。",
            code("Closure"),
            " が破棄された後、JS 内のクロージャの使用はすべて例外を引き起こします。",
        ],
        p![
            code("Closure"),
            " は、",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/struct.Function.html",
                code("&js_sys::Function"),
            ),
            " 型を受け取る ",
            code("js-sys"),
            " または ",
            code("web-sys"),
            " API を使用する際に一般的に使用されます。Yew で ",
            code("Closure"),
            " を使用する例は、",
            doc_link!(crate::pages::concepts::html::events, #"using-closure-verbose", "Using Closure セクション"),
            " の ",
            doc_link!(crate::pages::concepts::html::events, "Events"),
            " ページにあります。",
        ],
        p![
            italic![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html",
                    "Closure ドキュメント",
                ),
                "。",
            ],
        ],
        h2![
            link!("https://crates.io/crates/js-sys", "js-sys"),
        ],
        p![
            code("js-sys"),
            " クレートは、JavaScript の標準組み込みオブジェクトのバインディング/インポートを提供します。これには、それらのメソッドやプロパティが含まれます。",
        ],
        p![
            "これは Web API を含みません。Web API は ",
            doc_link!(crate::pages::concepts::basic_web_technologies::web_sys, code("web-sys")),
            " の役割です！",
        ],
        p![
            italic![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/index.html",
                    "js-sys ドキュメント",
                ),
                "。",
            ],
        ],
        h2![
            link!("https://crates.io/crates/wasm-bindgen-futures", "wasm-bindgen-futures"),
        ],
        p![
            code("wasm-bindgen-futures"),
            " クレートは、JavaScript の Promise 型を Rust の ",
            link!("https://doc.rust-lang.org/stable/std/future/trait.Future.html", code("Future")),
            " として扱うためのブリッジを提供し、Rust の Future を JavaScript の Promise に変換するユーティリティを含みます。Rust（wasm）で非同期または他のブロッキング作業を処理する際に役立ち、JavaScript のイベントや JavaScript I/O プリミティブと対話する能力を提供します。",
        ],
        p!["現在、このクレートには3つの主要なインターフェースがあります："],
        ol![
            li_blocks![
                p![
                    link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/struct.JsFuture.html",
                        code("JsFuture"),
                    ),
                    " - ",
                    link!("https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/struct.Promise.html",
                        code("Promise"),
                    ),
                    " を使用して構築された型で、",
                    code("Future<Output=Result<JsValue, JsValue>>"),
                    " として使用できます。",
                    code("Promise"),
                    " が解決されると、この ",
                    code("Future"),
                    " は ",
                    code("Ok"),
                    " に解決され、",
                    code("Promise"),
                    " が拒否されると ",
                    code("Err"),
                    " に解決され、それぞれ ",
                    code("Promise"),
                    " の解決または拒否の値を含みます。",
                ],
            ],
            li_blocks![
                p![
                    link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.future_to_promise.html",
                        code("future_to_promise"),
                    ),
                    " - Rust の ",
                    code("Future<Output=Result<JsValue, JsValue>>"),
                    " を JavaScript の ",
                    code("Promise"),
                    " に変換します。Future の結果は、JavaScript 内の解決または拒否された ",
                    code("Promise"),
                    " に変換されます。",
                ],
            ],
            li_blocks![
                p![
                    link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html",
                        code("spawn_local"),
                    ),
                    " - 現在のスレッドで ",
                    code("Future<Output = ()>"),
                    " を生成します。これは、Rust 内で Future を実行する最良の方法であり、JavaScript に送信するのではなく、Rust 内で実行します。",
                ],
            ],
        ],
        p![
            italic![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/index.html",
                    "wasm-bindgen-futures ドキュメント",
                ),
                "。",
            ],
        ],
        h3![
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html",
                "spawn_local",
            ),
        ],
        p![
            code("spawn_local"),
            " は、非同期 API を使用するライブラリを使用する際に、Yew で ",
            code("wasm-bindgen-futures"),
            " クレートの最も一般的に使用される部分です。",
        ],
        code_block("rust", "use web_sys::console;
use wasm_bindgen_futures::spawn_local;

async fn my_async_fn() -> String { String::from(\"Hello\") }

spawn_local(async {
    let mut string = my_async_fn().await;
    string.push_str(\", world!\");
    // \"Hello, world!\" を出力します
    console::log_1(&string.into());
});"),
        p![
            "Yew はいくつかの API に futures のサポートを追加しており、特に ",
            code("async"),
            " ブロックを受け入れる ",
            code("callback_future"),
            " を作成できることが注目されます。これは内部的に ",
            code("spawn_local"),
            " を使用しています。",
        ],
        p![
            italic![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html",
                    "spawn_local ドキュメント",
                ),
                "。",
            ],
        ],
    ])
}

crate::doc_page!(
    "wasm-bindgen",
    "/ja/docs/concepts/basic-web-technologies/wasm-bindgen",
    page_content()
);
