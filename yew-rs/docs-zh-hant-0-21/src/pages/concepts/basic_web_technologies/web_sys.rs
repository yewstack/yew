crate::doc_page!(
    "web-sys",
    "/zh-Hant/docs/concepts/basic-web-technologies/web-sys",
    Content::new(vec![
        p![
            link!(
                "https://crates.io/crates/web-sys",
                text("web-sys crate"),
            ),
            text(
                " 為 Web API 提供綁定。這是從瀏覽器 WebIDL \
                 產生的，這就是為什麼有些名稱如此之長，有些類型如此模糊的原因。",
            ),
        ],
        h2![text("`web-sys` 中的特性 (features)")],
        p![
            code("web-sys"),
            text(
                " crate 中啟用了所有特性可能會為 Wasm \
                 應用程式增加很多冗餘。為了解決這個問題，大多數類型都是透過啟用 features \
                 進行控制的，這樣你只需要包含你的應用程式所需的類型。 Yew 啟用了 ",
            ),
            code("web-sys"),
            text(
                " 的幾個特性，並在其公共 API 中公開了一些類型。你通常需要自行將 ",
            ),
            code("web-sys"),
            text(" 新增為依賴項。"),
        ],
        h2![text("`web-sys` 中的繼承")],
        p![
            text("在"),
            link!("", text("模擬繼承")),
            text(
                "部分，你可以了解到 Rust 通常提供了一種模擬 JavaScript 中繼承的方法。這在 ",
            ),
            code("web-sys"),
            text(
                " 中非常重要，因為了解一個類型上有哪些方法意味著了解它的繼承。",
            ),
        ],
        p![
            text("這一部分將查看一個特定的元素，並使用Rust 呼叫"),
            link!(
                "https://doc.rust-lang.org/std/ops/trait.Deref.html#tymethod.deref",
                text("Deref::deref"),
            ),
            text(" 列出其繼承，直到該值為"),
            link!("", text("JsValue")),
            text("。"),
        ],
        code_block(
            "rust",
            r#"use std::ops::Deref;
use web_sys::{
    Element,
    EventTarget,
    HtmlElement,
    HtmlTextAreaElement,
    Node,
};

fn inheritance_of_text_area(text_area: HtmlTextAreaElement) {
    // HtmlTextAreaElement 是 HTML 中的 <textarea>。
    let html_element: &HtmlElement = text_area.deref();

    let element: &Element = html_element.deref();

    let node: &Node = element.deref();

    let event_target: &EventTarget = node.deref();

    // 注意我們現在已經從 web-sys 類型轉移到內建的 JavaScript 類型，
    // 這些類型在 js-sys crate 中。
    let object: &js_sys::Object = event_target.deref();

    // 注意我們現在已經從 js-sys 類型轉移到 wasm-bindgen crate 中的根 JsValue。
    let js_value: &wasm_bindgen::JsValue = object.deref();

    // 這樣使用 deref 意味著我們必須手動遍歷繼承樹。
    // 但是，您可以在 HtmlTextAreaElement 類型上呼叫 JsValue 方法。
    assert!(!text_area.is_string());

    // 這個空函數只是為了證明我們可以將 HtmlTextAreaElement 作為 &EventTarget 傳遞。
    fn this_function_only_takes_event_targets(targets: &EventTarget) {};

    // 編譯器將沿著 deref 鏈向下走，以符合這裡的類型。
    this_function_only_takes_event_targets(&text_area);

    // AsRef 實作可讓您將 HtmlTextAreaElement 視為 &EventTarget。
    let event_target: &EventTarget = text_area.as_ref();

}"#,
        ),
        p![
            text("_"),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/web-sys/inheritance.html",
                text("wasm-bindgen 指引中的 web-sys 繼承"),
            ),
            text("_"),
        ],
        h2![text("`NodeRef` 中的 `Node`")],
        p![
            text("Yew 使用 "),
            link!(
                "concepts/function-components/node-refs.mdx",
                text("NodeRef"),
            ),
            text(" 來提供一種方式來保留由 "),
            link!(
                "concepts/html/introduction.mdx",
                text("html!"),
            ),
            text(" 巨集所建立的 "),
            code("Node"),
            text(" 的引用。 "),
            code("NodeRef"),
            text(" 中的 "),
            code("Node"),
            text(" 指的是 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Node.html",
                text("web_sys::Node"),
            ),
            text("。 "),
            code("NodeRef::get"),
            text(" 方法將傳回一個 "),
            code("Option<Node>"),
            text(
                " 值，但是，在 Yew \
                 中，大多數情況下，您希望將此值轉換為特定元素，以便使用其特定方法。如果存在，可以使用 ",
            ),
            link!("", text("JsCast")),
            text(" 對 "),
            code("Node"),
            text(" 值進行轉換，但是Yew 提供了 "),
            code("NodeRef::cast"),
            text(
                " 方法來執行此轉換，以方便使用，因此您不一定需要為 ",
            ),
            code("JsCast"),
            text(" 特性包含 "),
            code("wasm-bindgen"),
            text(" 依賴項。"),
        ],
        p![
            text("下面的兩個程式碼區塊本質上是相同的，第一個使用 "),
            code("NodeRef::cast"),
            text("，第二個使用 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into",
                text("JsCast::dyn_into"),
            ),
            text(" 在 "),
            code("NodeRef::get"),
            text(" 傳回的 "),
            code("web_sys::Node"),
            text(" 上。"),
        ],
        h2![text("JavaScript 重構為 Rust 的範例")],
        p![text(
            "這一節展示如何將與 Web API 互動的 JavaScript 程式碼範例重寫為 Rust 中的 ",
        ), code("web-sys"), text("。")],
        h3![text("JavaScript 範例")],
        code_block(
            "js",
            r#"document.getElementById('mousemoveme').onmousemove = (e) => {
    // e 為滑鼠事件對象
    var rect = e.target.getBoundingClientRect()
    var x = e.clientX - rect.left // 元素内的 x 位置。
    var y = e.clientY - rect.top // 元素内的 y 位置。
    console.log('Left? : ' + x + ' ; Top? : ' + y + '.')
}"#,
        ),
        h3![text("用 `web-sys` 重寫的範例")],
        p![
            text("只使用 "),
            code("web-sys"),
            text("，上面的 JavaScript 範例可以這樣實作："),
        ],
        code_block(
            "toml",
            r#"[dependencies]
wasm-bindgen = "0.2"

[dependencies.web-sys]
version = "0.3"
# 需要啟用所有我們想要使用的 web-sys 功能！
features = [
    "console",
    "Document",
    "HtmlElement",
    "MouseEvent",
    "DomRect",
]"#,
        ),
        code_block(
            "rust",
            r#"use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{console, Document, HtmlElement, MouseEvent};

let mousemove = Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(|e| {
    let rect = e
        .target()
        .expect("mouse event doesn't have a target")
        .dyn_into::<HtmlElement>()
        .expect("event target should be of type HtmlElement")
        .get_bounding_client_rect();
    let x = (e.client_x() as f64) - rect.left();
    let y = (e.client_y() as f64) - rect.top();
    console::log_1(&format!("Left? : {} ; Top? : {}", x, y).into());
}));

Document::new()
    .expect("global document not set")
    .get_element_by_id("mousemoveme")
    .expect("element with id `mousemoveme` not present")
    .unchecked_into::<HtmlElement>()
    .set_onmousemove(mousemove.as_ref().dyn_ref());

// 我們現在需要保存 `mousemove` 閉包，以便在事件觸發時閉包仍然在記憶體中。"#,
        ),
        p![text(
            "這個版本更加冗長，但你可能會注意到其中的一部分是由於失敗類型提醒我們，一些函數呼叫有必須保持的不變量，否則將在 \
             Rust 中引發 panic。另一個冗長的部分是呼叫 ",
        ), code("JsCast"), text(" 來將不同類型轉換為特定類型，以便呼叫其特定方法。")],
        h3![text("用 Yew 重寫的範例")],
        p![
            text("在Yew 中，您將主要建立 "),
            link!(
                "concepts/function-components/callbacks.mdx",
                text("Callback"),
            ),
            text(" 以在 "),
            link!(
                "concepts/html/introduction.mdx",
                text("html!"),
            ),
            text(
                " 巨集中使用，因此範例將使用這種方法，而不是完全複製上面的方法：",
            ),
        ],
        code_block(
            "toml",
            r#"[dependencies.web-sys]
version = "0.3"
# 我們需要啟用 `DomRect` 特性以使用 `get_bounding_client_rect` 方法。
features = [
    "console",
    "HtmlElement",
    "MouseEvent",
    "DomRect",
]
"#,
        ),
        code_block(
            "rust",
            r#"use web_sys::{console, HtmlElement, MouseEvent};
use yew::{
    html,
    Callback, TargetCast,
};

let onmousemove = Callback::from(|e: MouseEvent| {
    if let Some(target) = e.target_dyn_into::<HtmlElement>() {
        let rect = target.get_bounding_client_rect();
        let x = (e.client_x() as f64) - rect.left();
        let y = (e.client_y() as f64) - rect.top();
        console::log_1(&format!("Left? : {} ; Top? : {}", x, y).into());
    }
});

html! {
    <div id="mousemoveme" {onmousemove}></div>
};"#,
        ),
        h2![text("補充依賴庫")],
        p![
            code("web-sys"),
            text(
                " 是 Web API 的原始綁定，囙此在 Rust 中會有一些痛苦，因為它並不是為 Rust \
                 或甚至强類型系統設計的，這就是社區 crate 提供了對 ",
            ),
            code("web-sys"),
            text(" 的抽象，以提供更符合 Rust 習慣的 API。"),
        ],
        p![text(
            "_[補充依賴庫清單]（/community/external-libs）_",
        )],
    ])
);
