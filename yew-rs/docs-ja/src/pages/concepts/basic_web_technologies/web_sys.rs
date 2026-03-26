pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            link!("https://crates.io/crates/web-sys",
                "web-sys クレート",
            ),
            " は Web API のバインディングを提供します。これはブラウザの WebIDL から生成されるため、名前が長くなったり、型が曖昧になったりすることがあります。",
        ],
        h2!["`web-sys` の特性 (features)"],
        p![
            code("web-sys"),
            " クレートで全ての特性を有効にすると、Wasm アプリケーションに多くの冗長性が追加される可能性があります。この問題を解決するために、ほとんどの型は特性を有効にすることで制御され、アプリケーションに必要な型だけを含めることができます。Yew は ",
            code("web-sys"),
            " のいくつかの特性を有効にし、その公開 API でいくつかの型を公開しています。通常、",
            code("web-sys"),
            " を依存関係として追加する必要があります。",
        ],
        h2!["`web-sys` の継承"],
        p![
            link!("/ja/docs/concepts/basic-web-technologies/wasm-bindgen#simulating-inheritance",
                "継承のシミュレーション",
            ),
            "のセクションでは、Rust が通常 JavaScript の継承をシミュレートする方法を提供していることがわかります。これは ",
            code("web-sys"),
            " で非常に重要です。ある型にどのようなメソッドがあるかを理解するためには、その継承を理解する必要があります。",
        ],
        p![
            "このセクションでは、特定の要素を見て、Rust で ",
            link!("https://doc.rust-lang.org/std/ops/trait.Deref.html#tymethod.deref",
                code("Deref::deref"),
            ),
            " を呼び出して、その値が ",
            link!("/ja/docs/concepts/basic-web-technologies/wasm-bindgen#jsvalue",
                code("JsValue"),
            ),
            " になるまでの継承をリストします：",
        ],
        code_block("rust", "use std::ops::Deref;
use web_sys::{
    Element,
    EventTarget,
    HtmlElement,
    HtmlTextAreaElement,
    Node,
};

fn inheritance_of_text_area(text_area: HtmlTextAreaElement) {
    // HtmlTextAreaElement は HTML の <textarea> です。
    let html_element: &HtmlElement = text_area.deref();

    let element: &Element = html_element.deref();

    let node: &Node = element.deref();

    let event_target: &EventTarget = node.deref();

    // 注意: ここで web-sys タイプから js-sys クレート内の組み込み JavaScript タイプに移行しました。
    let object: &js_sys::Object = event_target.deref();

    // 注意: ここで js-sys タイプから wasm-bindgen クレートのルート JsValue に移行しました。
    let js_value: &wasm_bindgen::JsValue = object.deref();

    // このように deref を使用することで、継承ツリーを手動でたどる必要があります。
    // しかし、HtmlTextAreaElement タイプで JsValue メソッドを呼び出すことができます。
    assert!(!text_area.is_string());

    // この空の関数は、HtmlTextAreaElement を &EventTarget として渡すことができることを示すためのものです。
    fn this_function_only_takes_event_targets(targets: &EventTarget) {};

    // コンパイラはここでタイプを一致させるために deref チェーンを下にたどります。
    this_function_only_takes_event_targets(&text_area);

    // AsRef 実装により、HtmlTextAreaElement を &EventTarget として扱うことができます。
    let event_target: &EventTarget = text_area.as_ref();

}"),
        p![
            link!("https://wasm-bindgen.github.io/wasm-bindgen/web-sys/inheritance.html",
                "wasm-bindgen ガイドの web-sys 継承",
            ),
        ],
        h2!["`NodeRef` の `Node`"],
        p![
            "Yew は ",
            link!("/ja/docs/concepts/function-components/node-refs", code("NodeRef")),
            " を使用して、",
            link!("/ja/docs/concepts/html", code("html!")),
            " マクロによって作成された ",
            code("Node"),
            " の参照を保持する方法を提供します。",
            code("NodeRef"),
            " の ",
            code("Node"),
            " は ",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Node.html",
                code("web_sys::Node"),
            ),
            " を指します。",
            code("NodeRef::get"),
            " メソッドは ",
            code("Option<Node>"),
            " 値を返しますが、Yew ではほとんどの場合、この値を特定の要素に変換して、その特定のメソッドを使用することを望みます。存在する場合、",
            link!("/ja/docs/concepts/basic-web-technologies/wasm-bindgen#jscast",
                code("JsCast"),
            ),
            " を使用して ",
            code("Node"),
            " 値を変換できますが、Yew はこの変換を実行するための ",
            code("NodeRef::cast"),
            " メソッドを提供しているため、",
            code("JsCast"),
            " 特性のために ",
            code("wasm-bindgen"),
            " 依存関係を含める必要はありません。",
        ],
        p![
            "以下の2つのコードブロックは本質的に同じです。最初のものは ",
            code("NodeRef::cast"),
            " を使用し、2 番目のものは ",
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into",
                code("JsCast::dyn_into"),
            ),
            " を使用して ",
            code("NodeRef::get"),
            " が返す ",
            code("web_sys::Node"),
            " 上で呼び出しています。",
        ],
        tabs(
            "Using NodeRef::cast",
            vec![
                tab(
                    "Using NodeRef::cast",
                    "Using NodeRef::cast",
                    vec![code_block(
                        "rust",
                        r#"use web_sys::HtmlInputElement;
use yew::NodeRef;

fn with_node_ref_cast(node_ref: NodeRef) {
    if let Some(input) = node_ref.cast::<HtmlInputElement>() {
        // HtmlInputElement をここで処理します
    }
}"#,
                    )],
                ),
                tab(
                    "Using NodeRef::get",
                    "Using NodeRef::get",
                    vec![code_block(
                        "rust",
                        r#"use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::NodeRef;

fn with_jscast(node_ref: NodeRef) {
    if let Some(input) = node_ref
        .get()
        .and_then(|node| node.dyn_into::<HtmlInputElement>().ok()) {
        // HtmlInputElement をここで処理します
    }
}"#,
                    )],
                ),
            ],
        ),
        h2!["Rust にリファクタリングされた JavaScript の例"],
        p![
            "このセクションでは、Web API と対話する JavaScript コードの例を Rust の ",
            code("web-sys"),
            " にリファクタリングする方法を示します。",
        ],
        h3!["JavaScript の例"],
        code_block("js", "document.getElementById('mousemoveme').onmousemove = (e) => {
    // e はマウスイベントオブジェクトです
    var rect = e.target.getBoundingClientRect()
    var x = e.clientX - rect.left // 要素内の x 位置。
    var y = e.clientY - rect.top // 要素内の y 位置。
    console.log('Left? : ' + x + ' ; Top? : ' + y + '.')
}"),
        h3!["`web-sys` を使用して書き直した例"],
        p![
            code("web-sys"),
            " のみを使用して、上記の JavaScript の例は次のように実装できます：",
        ],
        code_block_title("toml", "Cargo.toml", "[dependencies]
wasm-bindgen = \"0.2\"

[dependencies.web-sys]
version = \"0.3\"
# 使用したいすべての web-sys 機能を有効にする必要があります！
features = [
    \"console\",
    \"Document\",
    \"HtmlElement\",
    \"MouseEvent\",
    \"DomRect\",
]"),
        code_block("rust", "use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{console, Document, HtmlElement, MouseEvent};

let mousemove = Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(|e| {
    let rect = e
        .target()
        .expect(\"mouse event doesn't have a target\")
        .dyn_into::<HtmlElement>()
        .expect(\"event target should be of type HtmlElement\")
        .get_bounding_client_rect();
    let x = (e.client_x() as f64) - rect.left();
    let y = (e.client_y() as f64) - rect.top();
    console::log_1(&format!(\"Left? : {} ; Top? : {}\", x, y).into());
}));

Document::new()
    .expect(\"global document not set\")
    .get_element_by_id(\"mousemoveme\")
    .expect(\"element with id `mousemoveme` not present\")
    .unchecked_into::<HtmlElement>()
    .set_onmousemove(mousemove.as_ref().dyn_ref());

// 現在、イベントが発生したときにクロージャがメモリに残るように、`mousemove` クロージャを保存する必要があります。"),
        p![
            "このバージョンはより冗長ですが、その一部は失敗した型が私たちにいくつかの関数呼び出しに保持しなければならない不変条件を思い出させるためです。これらの不変条件が守られないと、Rust ではパニックが発生します。もう一つの冗長な部分は、特定のメソッドを呼び出すために異なる型を特定の型に変換するための ",
            code("JsCast"),
            " の呼び出しです。",
        ],
        h3!["Yew で書き直した例"],
        p![
            "Yew では、主に ",
            link!("/ja/docs/concepts/function-components/callbacks", code("Callback")),
            " を作成して ",
            link!("/ja/docs/concepts/html", code("html!")),
            " マクロで使用するため、例はこの方法を使用します。上記の方法を完全にコピーするのではなく、この方法を使用します：",
        ],
        code_block_title("toml", "Cargo.toml", "[dependencies.web-sys]
version = \"0.3\"
# `get_bounding_client_rect` メソッドを使用するには、`DomRect` 特性を有効にする必要があります。
features = [
    \"console\",
    \"HtmlElement\",
    \"MouseEvent\",
    \"DomRect\",
]"),
        code_block("rust", "use web_sys::{console, HtmlElement, MouseEvent};
use yew::{
    html,
    Callback, TargetCast,
};

let onmousemove = Callback::from(|e: MouseEvent| {
    if let Some(target) = e.target_dyn_into::<HtmlElement>() {
        let rect = target.get_bounding_client_rect();
        let x = (e.client_x() as f64) - rect.left();
        let y = (e.client_y() as f64) - rect.top();
        console::log_1(&format!(\"Left? : {} ; Top? : {}\", x, y).into());
    }
});

html! {
    <div id=\"mousemoveme\" {onmousemove}></div>
};"),
        h2!["追加の依存ライブラリ"],
        p![
            code("web-sys"),
            " は Web API の生のバインディングであるため、Rust で使用する際にはいくつかの困難が伴います。これは、",
            code("web-sys"),
            " が Rust や強い型システムのために設計されていないためです。そこで、コミュニティのクレートが ",
            code("web-sys"),
            " に対する抽象化を提供し、Rust の慣習により適した API を提供しています。",
        ],
        p![
            italic![
                link!("/community/external-libs", "追加の依存ライブラリ一覧"),
            ],
        ],
    ])
}

crate::doc_page!(
    "web-sys",
    "/ja/docs/concepts/basic-web-technologies/web-sys",
    page_content()
);
