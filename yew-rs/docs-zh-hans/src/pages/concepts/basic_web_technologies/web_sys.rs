pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![
            link(
                "https://crates.io/crates/web-sys",
                vec![text("web-sys crate")],
            ),
            text(
                " 为 Web API 提供绑定。这是从浏览器 WebIDL \
                 生成的，这就是为什么有些名称如此之长，有些类型如此模糊的原因。",
            ),
        ]),
        h2(vec![text("`web-sys` 中的特性 (features)")]),
        p(vec![
            code("web-sys"),
            text(
                " crate 中启用了所有特性可能会给 Wasm \
                 应用程序增加很多冗余。为了解决这个问题，大多数类型都是通过启用 features \
                 进行控制的，这样你只需要包含你的应用程序所需的类型。Yew 启用了 ",
            ),
            code("web-sys"),
            text(
                " 的几个特性，并在其公共 API 中公开了一些类型。你通常需要自行将 ",
            ),
            code("web-sys"),
            text(" 添加为依赖项。"),
        ]),
        h2(vec![text("`web-sys` 中的继承")]),
        p(vec![
            text("在"),
            link("/zh-Hans/docs/concepts/basic-web-technologies/wasm-bindgen#simulating-inheritance", vec![text("模拟继承")]),
            text(
                "部分，你可以了解到 Rust 通常提供了一种模拟 JavaScript 中继承的方法。这在 ",
            ),
            code("web-sys"),
            text(
                " 中非常重要，因为了解一个类型上有哪些方法意味着了解它的继承。",
            ),
        ]),
        p(vec![
            text("这一部分将查看一个特定的元素，并使用 Rust 调用 "),
            link(
                "https://doc.rust-lang.org/std/ops/trait.Deref.html#tymethod.deref",
                vec![text("Deref::deref")],
            ),
            text(" 列出其继承，直到该值为 "),
            link("/zh-Hans/docs/concepts/basic-web-technologies/wasm-bindgen#jsvalue", vec![text("JsValue")]),
            text("。"),
        ]),
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

    // 注意我们现在已经从 web-sys 类型转移到内置的 JavaScript 类型，
    // 这些类型在 js-sys crate 中。
    let object: &js_sys::Object = event_target.deref();

    // 注意我们现在已经从 js-sys 类型转移到 wasm-bindgen crate 中的根 JsValue。
    let js_value: &wasm_bindgen::JsValue = object.deref();

    // 这样使用 deref 意味着我们必须手动遍历继承树。
    // 但是，您可以在 HtmlTextAreaElement 类型上调用 JsValue 方法。
    assert!(!text_area.is_string());

    // 这个空函数只是为了证明我们可以将 HtmlTextAreaElement 作为 &EventTarget 传递。
    fn this_function_only_takes_event_targets(targets: &EventTarget) {};

    // 编译器将沿着 deref 链向下走，以匹配这里的类型。
    this_function_only_takes_event_targets(&text_area);

    // AsRef 实现允许您将 HtmlTextAreaElement 视为 &EventTarget。
    let event_target: &EventTarget = text_area.as_ref();

}"#,
        ),
        p(vec![
            text("_"),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/web-sys/inheritance.html",
                vec![text("wasm-bindgen 指引中的 web-sys 继承")],
            ),
            text("_"),
        ]),
        h2(vec![text("`NodeRef` 中的 `Node`")]),
        p(vec![
            text("Yew 使用 "),
            link(
                "/zh-Hans/docs/concepts/function-components/node-refs",
                vec![text("NodeRef")],
            ),
            text(" 来提供一种方式来保留由 "),
            link(
                "/zh-Hans/docs/concepts/html",
                vec![text("html!")],
            ),
            text(" 宏创建的 "),
            code("Node"),
            text(" 的引用。"),
            code("NodeRef"),
            text(" 中的 "),
            code("Node"),
            text(" 指的是 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Node.html",
                vec![text("web_sys::Node")],
            ),
            text("。"),
            code("NodeRef::get"),
            text(" 方法将返回一个 "),
            code("Option<Node>"),
            text(" 值，但是，在 Yew 中，大多数情况下，您希望将此值转换为特定元素，以便使用其特定方法。如果存在，可以使用 "),
            link("/zh-Hans/docs/concepts/basic-web-technologies/wasm-bindgen#jscast", vec![text("JsCast")]),
            text(" 对 "),
            code("Node"),
            text(" 值进行转换，但是 Yew 提供了 "),
            code("NodeRef::cast"),
            text(" 方法来执行此转换，以方便使用，因此您不一定需要为 "),
            code("JsCast"),
            text(" 特性包含 "),
            code("wasm-bindgen"),
            text(" 依赖项。"),
        ]),
        p(vec![
            text("下面的两个代码块本质上是相同的，第一个使用 "),
            code("NodeRef::cast"),
            text("，第二个使用 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into",
                vec![text("JsCast::dyn_into")],
            ),
            text(" 在 "),
            code("NodeRef::get"),
            text(" 返回的 "),
            code("web_sys::Node"),
            text(" 上。"),
        ]),
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
        // 在这里处理 HtmlInputElement
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
        // 在这里处理 HtmlInputElement
    }
}"#,
                    )],
                ),
            ],
        ),
        h2(vec![text("JavaScript 重构为 Rust 的示例")]),
        p(vec![
            text("这一节展示了如何将与 Web API 交互的 JavaScript 代码示例重写为 Rust 中的 "),
            code("web-sys"),
            text("。"),
        ]),
        h3(vec![text("JavaScript 示例")]),
        code_block(
            "js",
            r#"document.getElementById('mousemoveme').onmousemove = (e) => {
    // e 为鼠标事件对象
    var rect = e.target.getBoundingClientRect()
    var x = e.clientX - rect.left // 元素内的 x 位置。
    var y = e.clientY - rect.top // 元素内的 y 位置。
    console.log('Left? : ' + x + ' ; Top? : ' + y + '.')
}"#,
        ),
        h3(vec![text("用 `web-sys` 重写的示例")]),
        p(vec![
            text("仅使用 "),
            code("web-sys"),
            text("，上面的 JavaScript 示例可以这样实现："),
        ]),
        code_block_title(
            "toml",
            "Cargo.toml",
            r#"[dependencies]
wasm-bindgen = "0.2"

[dependencies.web-sys]
version = "0.3"
# 需要启用所有我们想要使用的 web-sys 特性！
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

// 我们现在需要保存 `mousemove` 闭包，以便在事件触发时闭包仍然在内存中。"#,
        ),
        p(vec![
            text("这个版本更加冗长，但你可能会注意到其中的一部分是由于失败类型提醒我们，一些函数调用有必须保持的不变量，否则将在 Rust 中引发 panic。另一个冗长的部分是调用 "),
            code("JsCast"),
            text(" 来将不同类型转换为特定类型，以便调用其特定方法。"),
        ]),
        h3(vec![text("用 Yew 重写的示例")]),
        p(vec![
            text("在 Yew 中，您将主要创建 "),
            link(
                "/zh-Hans/docs/concepts/function-components/callbacks",
                vec![text("Callback")],
            ),
            text(" 以在 "),
            link(
                "/zh-Hans/docs/concepts/html",
                vec![text("html!")],
            ),
            text(" 宏中使用，因此示例将使用这种方法，而不是完全复制上面的方法："),
        ]),
        code_block_title(
            "toml",
            "Cargo.toml",
            r#"[dependencies.web-sys]
version = "0.3"
# 我们需要启用 `DomRect` 特性以使用 `get_bounding_client_rect` 方法。
features = [
    "console",
    "HtmlElement",
    "MouseEvent",
    "DomRect",
]"#,
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
        h2(vec![text("补充依赖库")]),
        p(vec![
            code("web-sys"),
            text(
                " 是 Web API 的原始绑定，因此在 Rust \
                 中会有一些痛苦，因为它并不是为 Rust \
                 或甚至强类型系统设计的，这就是社区 crate 提供了对 ",
            ),
            code("web-sys"),
            text(" 的抽象，以提供更符合 Rust 习惯的 API。"),
        ]),
        p(vec![
            text("_"),
            link(
                "/community/external-libs",
                vec![text("补充依赖库清单")],
            ),
            text("_"),
        ]),
    ])
}

crate::doc_page!(
    "web-sys",
    "/zh-Hans/docs/concepts/basic-web-technologies/web-sys",
    page_content()
);
