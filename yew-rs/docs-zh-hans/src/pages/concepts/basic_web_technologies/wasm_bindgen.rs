pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![
            link(
                "https://github.com/rustwasm/wasm-bindgen",
                vec![text("wasm-bindgen")],
            ),
            text(
                " 是一个在 JavaScript 和 Rust 函数之间建立调用桥梁的库和工具。它是由 ",
            ),
            link(
                "https://rustwasm.github.io/",
                vec![text("Rust 和 WebAssembly 工作组")],
            ),
            text(" 使用 Rust 构建的。"),
        ]),
        p(vec![
            text("Yew 使用 "),
            code("wasm-bindgen"),
            text(" 通过一些 crate 与浏览器进行交互："),
        ]),
        ul(vec![
            li(vec![link(
                "https://crates.io/crates/js-sys",
                vec![text("js-sys")],
            )]),
            li(vec![link(
                "https://crates.io/crates/wasm-bindgen",
                vec![text("wasm-bindgen")],
            )]),
            li(vec![link(
                "https://crates.io/crates/wasm-bindgen-futures",
                vec![text("wasm-bindgen-futures")],
            )]),
            li(vec![link(
                "https://crates.io/crates/web-sys",
                vec![text("web-sys")],
            )]),
        ]),
        p(vec![
            text(
                "本节将从更抽象的层次上探讨这些 crate，以便更容易地理解和使用 Yew 中的 ",
            ),
            code("wasm-bindgen"),
            text(" API。要了解有关 "),
            code("wasm-bindgen"),
            text(" 及其相关 crate 的更深入指南，请查看 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/",
                vec![text("wasm-bindgen 指引")],
            ),
            text("。"),
        ]),
        p(vec![
            text("有关上述 crate 的文档，请查看 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html",
                vec![text("wasm-bindgen docs.rs")],
            ),
            text("。"),
        ]),
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p(vec![
                text("使用 "),
                code("wasm-bindgen"),
                text(" doc.rs 搜索来查找已使用 "),
                code("wasm-bindgen"),
                text(" 导入的浏览器 API 和 JavaScript 类型。"),
            ])],
        ),
        h2(vec![text(
            "[`wasm-bindgen`](https://crates.io/crates/wasm-bindgen)",
        )]),
        p(vec![
            text(
                "这个 crate 为上面的其他 crate 提供了许多构建块。在本节中，我们只会涵盖 ",
            ),
            code("wasm-bindgen"),
            text(
                " crate 的两个主要领域，即宏和一些您会一遍又一遍看到的类型/特性。",
            ),
        ]),
        h3(vec![text("`#[wasm_bindgen]` macro")]),
        p(vec![
            code("#[wasm_bindgen]"),
            text(
                " 宏提供了 Rust 和 JavaScript 之间的接口，提供了一个在两者之间进行转换的系统。使用这个宏更为高级，除非您要使用外部 JavaScript 库，否则不应该使用它。",
            ),
            code("js-sys"),
            text(" 和 "),
            code("web-sys"),
            text(" crate 为内置 JavaScript 类型和浏览器 API 提供了 "),
            code("wasm-bindgen"),
            text(" 定义。"),
        ]),
        p(vec![
            text("让我们通过一个简单的示例来使用 "),
            code("#[wasm-bindgen]"),
            text(" 宏来导入一些特定版本的 "),
            link(
                "https://developer.mozilla.org/en-US/docs/Web/API/Console/log",
                vec![text("console.log")],
            ),
            text(" 函数。"),
        ]),
        code_block(
            "rust",
            r#"use wasm_bindgen::prelude::*;

// 首先让我们手动绑定 `console.log`，而不使用 `web_sys` 的帮助。
// 在这里，我们手动编写 `#[wasm_bindgen]` 注解，我们程序的正确性取决于这些注解的正确性！
#[wasm_bindgen]
extern "C" {
    // 在这里使用 `js_namespace` 来绑定 `console.log(..)` 而不是只有 `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // `console.log` 是多态的，所以我们可以使用多个签名绑定它。
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // 多个参数也是可以的！
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// 使用导入的函数！
log("Hello from Rust!");
log_u32(42);
log_many("Logging", "many values!");"#,
        ),
        p(vec![
            text("_这个示例是基于 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/examples/console-log.html",
                vec![text("1.2 使用 console.log 的 wasm-bindgen 指引")],
            ),
            text(" 改编的。_"),
        ]),
        h3(vec![text("模拟继承")]),
        p(vec![text(
            "在 JavaScript 类之间的继承是 JavaScript 语言的核心特性，DOM（文档对象模型）是围绕它设计的。当使用 ",
        ), code("wasm-bindgen"), text(
            " 导入类型时，您还可以添加描述它们继承关系的属性。",
        )]),
        p(vec![
            text("在 Rust 中，这种继承关系使用 "),
            link(
                "https://doc.rust-lang.org/std/ops/trait.Deref.html",
                vec![text("Deref")],
            ),
            text(" 和 "),
            link(
                "https://doc.rust-lang.org/std/convert/trait.AsRef.html",
                vec![text("AsRef")],
            ),
            text(
                " 特性来表示。这里举个例子可能会有所帮助；假设您有三种类型 ",
            ),
            code("A"),
            text("、"),
            code("B"),
            text(" 和 "),
            code("C"),
            text("，其中 "),
            code("C"),
            text(" 扩展了 "),
            code("B"),
            text("，而 "),
            code("B"),
            text(" 又扩展了 "),
            code("A"),
            text("。"),
        ]),
        p(vec![
            text("在导入这些类型时，"),
            code("#[wasm-bindgen]"),
            text(" 宏将按照以下方式实现 "),
            code("Deref"),
            text(" 和 "),
            code("AsRef"),
            text(" 特性："),
        ]),
        ul(vec![
            li(vec![
                code("C"),
                text(" 可以 "),
                code("Deref"),
                text(" 到 "),
                code("B"),
            ]),
            li(vec![
                code("B"),
                text(" 可以 "),
                code("Deref"),
                text(" 到 "),
                code("A"),
            ]),
            li(vec![
                code("C"),
                text(" 可以被 "),
                code("AsRef"),
                text(" 到 "),
                code("B"),
            ]),
            li(vec![
                code("C"),
                text(" 和 "),
                code("B"),
                text(" 都可以被 "),
                code("AsRef"),
                text(" 到 "),
                code("A"),
            ]),
        ]),
        p(vec![
            text("这些实现允许您在 "),
            code("C"),
            text(" 的实例上调用 "),
            code("A"),
            text(" 的方法，并将 "),
            code("C"),
            text(" 用作 "),
            code("&B"),
            text(" 或 "),
            code("&A"),
            text("。"),
        ]),
        p(vec![
            text("需要注意的是，使用 "),
            code("#[wasm_bindgen]"),
            text(
                " 导入的每种类型都有相同的根类型，您可以将其视为上面示例中的 ",
            ),
            code("A"),
            text("，这种类型是 "),
            link("#jsvalue", vec![text("JsValue")]),
            text("，下面有它的部分。"),
        ]),
        p(vec![
            text("_"),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/reference/attributes/on-js-imports/extends.html",
                vec![text("wasm-bindgen 指引中的 extends 部分")],
            ),
            text("_"),
        ]),
        h3(vec![text(
            "[`JsValue`](https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html)",
        )]),
        p(vec![
            text("这是 JavaScript 拥有的对象的表示，这是 "),
            code("wasm-bindgen"),
            text(" 的根捕获类型。任何来自 "),
            code("wasm-bindgen"),
            text(" 的类型都是 "),
            code("JsValue"),
            text(
                "，这是因为 JavaScript 没有强类型系统，因此接受变量 ",
            ),
            code("x"),
            text(" 的任何函数都不定义其类型，因此 "),
            code("x"),
            text(" 可以是有效的 JavaScript 值；因此 "),
            code("JsValue"),
            text("。如果您正在使用接受 "),
            code("JsValue"),
            text(" 的导入函数或类型，那么任何导入的值在技术上都是有效的。"),
        ]),
        p(vec![
            code("JsValue"),
            text(
                " 可以被函数接受，但该函数可能仍然只接受某些类型，这可能会导致 panic - 因此在使用原始 ",
            ),
            code("wasm-bindgen"),
            text(
                " API 时，请检查导入的 JavaScript 的文档，以确定是否会在该值不是某种类型时引发异常（panic）。",
            ),
        ]),
        p(vec![
            text("_"),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html",
                vec![text("JsValue 文档")],
            ),
            text("。_"),
        ]),
        h3(vec![text(
            "[`JsCast`](https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html)",
        )]),
        p(vec![
            text("Rust 有一个强类型系统，而 JavaScript……没有😞。为了让 Rust 保持这些强类型但仍然方便，WebAssembly 工作组提出了一个非常巧妙的特性 "),
            code("JsCast"),
            text("。它的工作是帮助您从一个 JavaScript \"类型\" 转换到另一个 \"类型\"，这听起来很模糊，但它意味着如果您有一个类型，您知道它是另一个类型，那么您可以使用 "),
            code("JsCast"),
            text(" 的函数从一个类型跳到另一个类型。当使用 "),
            code("web-sys"),
            text("、"),
            code("wasm_bindgen"),
            text("、"),
            code("js-sys"),
            text(" 时，了解这个很好的特性 - 您会注意到许多类型将从这些 crate 中实现 "),
            code("JsCast"),
            text("。"),
        ]),
        p(vec![
            code("JsCast"),
            text(
                " 提供了转换的检查和不检查方法 - 因此在运行时，如果您不确定某个对象是什么类型，您可以尝试将其转换，这将返回可能的失败类型，如 ",
            ),
            link(
                "https://doc.rust-lang.org/std/option/enum.Option.html",
                vec![text("Option")],
            ),
            text(" 和 "),
            link(
                "https://doc.rust-lang.org/std/result/enum.Result.html",
                vec![text("Result")],
            ),
            text("。"),
        ]),
        p(vec![
            text("一个常见的例子是在 "),
            link("/zh-Hans/docs/concepts/basic-web-technologies/web-sys", vec![text("web-sys")]),
            text(" 中，当您尝试获取事件的目标时。您可能知道目标元素是什么，但 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html",
                vec![text("web_sys::Event")],
            ),
            text(" API 总是会返回一个 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.target",
                vec![text("Option<web_sys::EventTarget>")],
            ),
            text("。 您需要将其转换为元素类型，以便调用其方法。"),
        ]),
        code_block(
            "rust",
            r#"// 需要先导入这个 Trait
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, HtmlInputElement, HtmlSelectElement};

fn handle_event(event: Event) {
    let target: EventTarget = event
        .target()
        .expect("I'm sure this event has a target!");

    // 也许目标是一个选择元素？
    if let Some(select_element) = target.dyn_ref::<HtmlSelectElement>() {
        // 做点别的
        return;
    }

    // 如果它能确定不是一个选择元素，那么我可以肯定它是一个输入元素！
    let input_element: HtmlInputElement = target.unchecked_into();
}"#,
        ),
        p(vec![
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_ref",
                vec![text("dyn_ref")],
            ),
            text(" 方法是一个检查的转换，返回一个 "),
            code("Option<&T>"),
            text("，这意味着如果转换失败，则可以再次使用原始类型，因此返回 "),
            code("None"),
            text("。"),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into",
                vec![text("dyn_into")],
            ),
            text(" 方法将消耗 "),
            code("self"),
            text("，这是 Rust 中 "),
            code("into"),
            text(" 方法的约定，返回的类型是 "),
            code("Result<T, Self>"),
            text("。如果转换失败，则原始的 "),
            code("Self"),
            text(" 值将在 "),
            code("Err"),
            text(" 中返回。您可以再试一次或对原始类型进行其他操作。"),
        ]),
        p(vec![
            text("_"),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html",
                vec![text("JsCast documentation")],
            ),
            text("._"),
        ]),
        h3(vec![text(
            "[`Closure`](https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html)",
        )]),
        p(vec![
            code("Closure"),
            text(
                " 类型提供了一种将 Rust 闭包传递到 JavaScript 的方法，出于健全性原因，传递给 JavaScript 的闭包必须具有 ",
            ),
            code("'static"),
            text(" 生命周期。"),
        ]),
        p(vec![text(
            "这种类型是一个\"句柄\"，意味着每当它被丢弃时，它将使其引用的 JS 闭包无效。在 ",
        ), code("Closure"), text(
            " 被丢弃后，对 JS 中闭包的任何使用都将引发异常。",
        )]),
        p(vec![
            text("当您使用接受类型 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/struct.Function.html",
                vec![text("&js_sys::Function")],
            ),
            text(" 的 "),
            code("js-sys"),
            text(" 或 "),
            code("web-sys"),
            text(" API 时，通常会使用 "),
            code("Closure"),
            text("。在 "),
            link("/zh-Hans/docs/concepts/html/events", vec![text("Events")]),
            text(" 页面的 "),
            link("/zh-Hans/docs/concepts/html/events#using-closure-verbose", vec![text("Using Closure 部分")]),
            text(" 中可以找到在 Yew 中使用 "),
            code("Closure"),
            text(" 的示例。"),
        ]),
        p(vec![
            text("_"),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html",
                vec![text("Closure 文档")],
            ),
            text("._"),
        ]),
        h2(vec![text(
            "[`js-sys`](https://crates.io/crates/js-sys)",
        )]),
        p(vec![
            code("js-sys"),
            text(
                " crate 提供了 JavaScript 标准内置对象的绑定/导入，包括它们的方法和属性。",
            ),
        ]),
        p(vec![
            text("这不包括任何 Web API，因为这是 "),
            link("/zh-Hans/docs/concepts/basic-web-technologies/web-sys", vec![text("web-sys")]),
            text(" 的作用！"),
        ]),
        p(vec![
            text("_"),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/index.html",
                vec![text("js-sys 文档")],
            ),
            text("._"),
        ]),
        h2(vec![text(
            "[`wasm-bindgen-futures`](https://crates.io/crates/wasm-bindgen-futures)",
        )]),
        p(vec![
            code("wasm-bindgen-futures"),
            text(" crate 提供了一个桥梁，用于将 JavaScript Promise 类型作为 Rust "),
            link(
                "https://doc.rust-lang.org/stable/std/future/trait.Future.html",
                vec![text("Future")],
            ),
            text(
                " 进行处理，并包含将 Rust Future 转换为 JavaScript Promise 的实用程序。当在 Rust（wasm）中处理异步或其他阻塞工作时，这可能很有用，并提供了与 JavaScript 事件和 JavaScript I/O 原语进行交互的能力。",
            ),
        ]),
        p(vec![text(
            "目前这个 crate 中有三个主要接口：",
        )]),
        ol(vec![
            li_blocks(vec![
                p(vec![
                    link(
                        "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/struct.JsFuture.html",
                        vec![text("JsFuture")],
                    ),
                    text(" - 一个使用 "),
                    link(
                        "https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/struct.Promise.html",
                        vec![text("Promise")],
                    ),
                    text(" 构造的类型，然后可以用作 "),
                    code("Future<Output=Result<JsValue, JsValue>>"),
                    text("。如果 "),
                    code("Promise"),
                    text(" 被解析，这个 "),
                    code("Future"),
                    text(" 将解析为 "),
                    code("Ok"),
                    text("，如果 "),
                    code("Promise"),
                    text(" 被拒绝，则解析为 "),
                    code("Err"),
                    text("，分别包含 "),
                    code("Promise"),
                    text(" 的解析或拒绝值。"),
                ]),
            ]),
            li_blocks(vec![
                p(vec![
                    link(
                        "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.future_to_promise.html",
                        vec![text("future_to_promise")],
                    ),
                    text(" - 将 Rust "),
                    code("Future<Output=Result<JsValue, JsValue>>"),
                    text(" 转换为 JavaScript "),
                    code("Promise"),
                    text("。未来的结果将转换为 JavaScript 中的已解析或已拒绝 "),
                    code("Promise"),
                    text("。"),
                ]),
            ]),
            li_blocks(vec![
                p(vec![
                    link(
                        "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html",
                        vec![text("spawn_local")],
                    ),
                    text(" - 在当前线程上生成一个 "),
                    code("Future<Output = ()>"),
                    text("。这是在 Rust 中运行 Future 的最佳方法，而不是将其发送到 JavaScript。"),
                ]),
            ]),
        ]),
        p(vec![
            text("_"),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/index.html",
                vec![text("wasm-bindgen-futures 文档")],
            ),
            text("._"),
        ]),
        h3(vec![text(
            "[`spawn_local`](https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html)",
        )]),
        p(vec![
            code("spawn_local"),
            text(" 将是 Yew 中 "),
            code("wasm-bindgen-futures"),
            text(" crate 中最常用的部分，因为这有助于使用具有异步 API 的库。"),
        ]),
        code_block(
            "rust",
            r#"use web_sys::console;
use wasm_bindgen_futures::spawn_local;

async fn my_async_fn() -> String { String::from("Hello") }

spawn_local(async {
    let mut string = my_async_fn().await;
    string.push_str(", world!");
    // 控制台输出 "Hello, world!"
    console::log_1(&string.into());
});"#,
        ),
        p(vec![
            text("Yew 还在某些 API 中添加了对 futures 的支持，最值得注意的是您可以创建一个接受 "),
            code("async"),
            text(" 块的 "),
            code("callback_future"),
            text(" - 这在内部使用了 "),
            code("spawn_local"),
            text("。"),
        ]),
        p(vec![
            text("_"),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html",
                vec![text("spawn_local 文档")],
            ),
            text("._"),
        ]),
    ])
}

crate::doc_page!(
    "wasm-bindgen",
    "/zh-Hans/docs/concepts/basic-web-technologies/wasm-bindgen",
    page_content()
);
