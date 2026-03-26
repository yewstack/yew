crate::doc_page!(
    "wasm-bindgen",
    "/zh-Hant/docs/concepts/basic-web-technologies/wasm-bindgen",
    Content::new(vec![
        p![
            link!(
                "https://github.com/rustwasm/wasm-bindgen",
                "wasm-bindgen",
            ),
            " 是一個在 JavaScript 和 Rust 函數之間建立呼叫橋樑的函式庫和工具。它是由 ",
            link!(
                "https://rustwasm.github.io/",
                "Rust 和 WebAssembly 工作小組",
            ),
            " 使用 Rust 建構的。",
        ],
        p![
            "Yew 使用 ",
            code("wasm-bindgen"),
            " 透過一些 crate 與瀏覽器進行互動：",
        ],
        ul![
            li![link!(
                "https://crates.io/crates/js-sys",
                "js-sys",
            )],
            li![link!(
                "https://crates.io/crates/wasm-bindgen",
                "wasm-bindgen",
            )],
            li![link!(
                "https://crates.io/crates/wasm-bindgen-futures",
                "wasm-bindgen-futures",
            )],
            li![link!(
                "https://crates.io/crates/web-sys",
                "web-sys",
            )],
        ],
        p![
            "本節將從更抽象的層次探討這些 crate，以便更容易理解和使用 Yew 中的 ",
            code("wasm-bindgen"),
            " API。要了解有關 ",
            code("wasm-bindgen"),
            " 及其相關 crate 的更深入指南，請查看 ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/",
                "wasm-bindgen 指引",
            ),
            "。",
        ],
        p![
            "有關上述 crate 的文檔，請查看 ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html",
                "wasm-bindgen docs.rs",
            ),
            "。",
        ],
        admonition!(
            AdmonitionType::Tip,
            None,
            p![
                "使用 ",
                code("wasm-bindgen"),
                " doc.rs 搜尋來尋找已使用 ",
                code("wasm-bindgen"),
                " 匯入的瀏覽器 API 和 JavaScript 類型。",
            ]
        ),
        h2!["[`wasm-bindgen`](https://crates.io/crates/wasm-bindgen)"],
        p![
            "這個 crate 為上面的其他 crate 提供了許多構建塊。在本節中，我們只會涵蓋 ",
            code("wasm-bindgen"),
            " crate 的兩個主要領域，即巨集和一些您會一遍又一遍看到的類型/特性。",
        ],
        h3!["`#[wasm_bindgen]` macro"],
        p![
            code("#[wasm_bindgen]"),
            " 巨集提供了 Rust 和 JavaScript \
                 之間的接口，提供了一個在兩者之間進行轉換的系統。使用這個巨集更為高級，除非您要使用外部 \
                 JavaScript 函式庫，否則不應該使用它。 ",
            code("js-sys"),
            " 和 ",
            code("web-sys"),
            " crate 為內建 JavaScript 類型和瀏覽器 API 提供了 ",
            code("wasm-bindgen"),
            " 定義。",
        ],
        p![
            "讓我們透過一個簡單的範例來使用",
            code("#[wasm-bindgen]"),
            " 巨集來匯入一些特定版本的",
            link!(
                "https://developer.mozilla.org/en-US/docs/Web/ API/Console/log",
                "console.log",
            ),
            " 函數。",
        ],
        code_block(
            "rust",
            r#"use wasm_bindgen::prelude::*;

// 首先讓我們手動綁定 `console.log`，而不使用 `web_sys` 的幫助。
// 在這裡，我們手動寫 `#[wasm_bindgen]` 註解，我們程式的正確性取決於這些註解的正確性！
#[wasm_bindgen]
extern "C" {
    // 在這裡使用 `js_namespace` 來綁定 `console.log(..)` 而不是只有 `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // `console.log` 是多態的，所以我們可以使用多個簽章綁定它。
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // 多個參數也是可以的！
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// 使用導入的函數！
log("Hello from Rust!");
log_u32(42);
log_many("Logging", "many values!");"#,
        ),
        p![
            "_這個範例是基於 ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/examples/console-log.html",
                "1.2 使用 console.log 的 wasm-bindgen 指引",
            ),
            " 改編的。 _",
        ],
        h3!["模擬繼承"],
        p![
            "在 JavaScript 類別之間的繼承是 JavaScript 語言的核心特性，DOM（文件物件模型）是圍繞它設計的。當使用 ",
            code("wasm-bindgen"),
            " 匯入類型時，您也可以新增描述它們繼承關係的屬性。",
        ],
        p![
            "在Rust 中，這種繼承關係使用",
            link!(
                "https://doc.rust-lang.org/std/ops/trait.Deref.html",
                "Deref",
            ),
            "和",
            link!(
                "https://doc. rust-lang.org/std/convert/trait.AsRef.html",
                "AsRef",
            ),
            " 特性來表示。這裡舉個例子可能會有所幫助；假設您有三種類型 ",
            code("A"),
            "、",
            code("B"),
            " 和 ",
            code("C"),
            "，其中 ",
            code("C"),
            " 擴展了 ",
            code("B"),
            "，而 ",
            code("B"),
            " 又擴展了 ",
            code("A"),
            "。",
        ],
        p![
            "在匯入這些類型時，",
            code("#[wasm-bindgen]"),
            " 巨集將按照下列方式實作 ",
            code("Deref"),
            " 和 ",
            code("AsRef"),
            " 特性：",
        ],
        ul![
            li![
                code("C"),
                " 可以 ",
                code("Deref"),
                " 到 ",
                code("B"),
            ],
            li![
                code("B"),
                " 可以 ",
                code("Deref"),
                " 到 ",
                code("A"),
            ],
            li![
                code("C"),
                " 可以被 ",
                code("AsRef"),
                " 到 ",
                code("B"),
            ],
            li![
                code("C"),
                " 和 ",
                code("B"),
                " 都可以被 ",
                code("AsRef"),
                " 到 ",
                code("A"),
            ],
        ],
        p![
            "這些實作允許您在 ",
            code("C"),
            " 的實例上呼叫 ",
            code("A"),
            " 的方法，並將 ",
            code("C"),
            " 用作 ",
            code("&B"),
            " 或 ",
            code("&A"),
            "。",
        ],
        p![
            "需要注意的是，使用",
            code("#[wasm_bindgen]"),
            " 導入的每種類型都有相同的根類型，您可以將其視為上面範例中的",
            code("A"),
            "，這種類型是",
            link!("#jsvalue", "JsValue"),
            "，下面有它的部分。",
        ],
        p![
            "_",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/reference/attributes/on-js-imports/extends.html",
                "wasm-bindgen 指引中的 extends 部分",
            ),
            "_",
        ],
        h3!["[`JsValue`](https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html)"],
        p![
            "這是 JavaScript 擁有的物件的表示，這是 ",
            code("wasm-bindgen"),
            " 的根捕獲類型。任何來自",
            code("wasm-bindgen"),
            " 的型別都是",
            code("JsValue"),
            "，這是因為JavaScript 沒有強型別系統，因此接受變數",
            code("x"),
            " 的任何函數都不定義其型別，因此",
            code("x"),
            " 可以是有效的JavaScript 值；因此",
            code("JsValue"),
            "。如果您正在使用接受 ",
            code("JsValue"),
            " 的導入函數或類型，那麼任何導入的值在技術上都是有效的。",
        ],
        p![
            code("JsValue"),
            " 可以被函數接受，但該函數可能仍然只接受某些類型，這可能會導致panic - 因此在使用原始",
            code("wasm-bindgen"),
            " API 時，請檢查導入的JavaScript \
                 的文檔，以確定是否會在該值不是某種類型時引發異常（panic）。",
        ],
        p![
            "_",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html",
                "JsValue 文件",
            ),
            "。 _",
        ],
        h3!["[`JsCast`](https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html)"],
        p![
            "Rust 有一個強型別系統，而 JavaScript…沒有😞。為了讓 Rust 保持這些強型別但仍然方便，WebAssembly 工作小組提出了一個非常巧妙的特性 ",
            code("JsCast"),
            "。它的工作是幫助您從一個JavaScript \"類型\" 轉換到另一個\"類型\"，這聽起來很模糊，但它意味著如果您有一個類型，您知道它是另一個類型，那麼您可以使用",
            code("JsCast "),
            " 的函數從一個型別跳到另一個型別。當使用 ",
            code("web-sys"),
            "、",
            code("wasm_bindgen"),
            "、",
            code("js-sys"),
            " 時，了解這個很好的特性 - 您會注意到許多類型將從這些 crate 中實作 ",
            code("JsCast"),
            "。",
        ],
        p![
            code("JsCast"),
            " 提供了轉換的檢查和不檢查方法- 因此在運行時，如果您不確定某個物件是什麼類型，您可以嘗試將其轉換，這將返回可能的失敗類型，如",
            "[",
            code("Option"),
            "] (https://doc.rust-lang.org/std/option/enum.Option.html) 和",
            link!(
                "https://doc.rust-lang.org/std/result/enum.Result. html",
                "Result",
            ),
            "。",
        ],
        p![
            "一個常見的例子是在 ",
            link!("", "web-sys"),
            " 中，當您嘗試取得事件的目標時。您可能知道目標元素是什麼，但",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html",
                "web_sys::Event",
            ),
            " API 總是會回傳一個",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.target",
                " Option<web_sys::EventTarget>",
            ),
            "。 您需要將其轉換為元素類型，以便呼叫其方法。",
        ],
        code_block(
            "rust",
            r#"// 需要先導入這個 Trait
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, HtmlInputElement, HtmlSelectElement};

fn handle_event(event: Event) {
    let target: EventTarget = event
        .target()
        .expect("I'm sure this event has a target!");

    // 也許目標是一個選擇元素？
    if let Some(select_element) = target.dyn_ref::<HtmlSelectElement>() {
        // 做點別的
        return;
    }

    // 如果它能確定不是一個選擇元素，那麼我可以肯定它是一個輸入元素！
    let input_element: HtmlInputElement = target.unchecked_into();
}"#,
        ),
        p![
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_ref",
                "dyn_ref",
            ),
            " 方法是一個檢查的轉換，回傳一個",
            code("Option<&T>"),
            "，這表示如果轉換失敗，則可以再次使用原始類型，因此傳回",
            code("None"),
            "。 ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into",
                "dyn_into",
            ),
            " 方法將消耗",
            code("self"),
            "，這是Rust 中",
            code("into"),
            " 方法的約定，傳回的類型是",
            code("Result<T, Self>"),
            "。如果轉換失敗，則原始的 ",
            code("Self"),
            " 值將在 ",
            code("Err"),
            " 中傳回。您可以再試一次或對原始類型進行其他操作。",
        ],
        p![
            "_",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html",
                "JsCast documentation",
            ),
            "._",
        ],
        h3!["[`Closure`](https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html)"],
        p![
            code("Closure"),
            " 類型提供了一種將 Rust 閉包傳遞到 JavaScript 的方法，出於健全性原因，傳遞給 JavaScript 的閉包必須具有 ",
            code("'static"),
            " 生命週期。",
        ],
        p![
            "這種類型是一個\"句柄\"，這意味著每當它被丟棄時，它將使其引用的 JS 閉包無效。在 ",
            code("Closure"),
            " 被丟棄後，對 JS 中閉包的任何使用都會引發異常。",
        ],
        p![
            "當您使用接受型別",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/struct.Function.html",
                "&js_sys::Function",
            ),
            " 的",
            code("js-sys"),
            " 或",
            code("web-sys"),
            " API 時，通常會使用",
            code("Closure"),
            "。在",
            link!("", "Events"),
            " 頁面的",
            link!("", "Using Closure 部分"),
            " 中可以找到Yew 中使用",
            code("Closure"),
            " 的範例。",
        ],
        p![
            "_",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html",
                "Closure 文件",
            ),
            "._",
        ],
        h2!["[`js-sys`](https://crates.io/crates/js-sys)"],
        p![
            code("js-sys"),
            " crate 提供了 JavaScript 標準內建物件的綁定/導入，包括它們的方法和屬性。",
        ],
        p![
            "這不包括任何 Web API，因為這是 ",
            link!("", "web-sys"),
            " 的作用！",
        ],
        p![
            "_",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/index.html",
                "js-sys 文件",
            ),
            "._",
        ],
        h2!["[`wasm-bindgen-futures`](https://crates.io/crates/wasm-bindgen-futures)"],
        p![
            code("wasm-bindgen-futures"),
            " crate 提供了一個橋樑，用於將JavaScript Promise 類型作為Rust ",
            link!(
                "https://doc.rust-lang.org/stable/std/future/trait.Future.html",
                "Future",
            ),
            " 進行處理，並包含將Rust Future 轉換為JavaScript Promise 的實用程式。當在 Rust（wasm）中處理非同步或其他阻塞工作時，這可能很有用，並提供了與 JavaScript 事件和 JavaScript I/O 原語互動的能力。",
        ],
        p!["目前這個 crate 中有三個主要介面："],
        ol![li![
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/struct.JsFuture.html",
                "JsFuture",
            ),
            " -",
        ]],
        p![
            "一個使用",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/struct.Promise.html",
                "Promise",
            ),
            " 建構的類型，然後可以用作",
            code("Future<Output=Result<JsValue, JsValue >>"),
            "。如果 ",
            code("Promise"),
            " 被解析，這個 ",
            code("Future"),
            " 將解析為 ",
            code("Ok"),
            "，如果 ",
            code("Promise"),
            " 被拒絕，則解析為 ",
            code("Err"),
            "，分別包含 ",
            code("Promise"),
            " 的解析或拒絕值。",
        ],
        ol![li![
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.future_to_promise.html",
                "future_to_promise",
            ),
            " -",
        ]],
        p![
            "將 Rust ",
            code("Future<Output=Result<JsValue, JsValue>>"),
            " 轉換為 JavaScript ",
            code("Promise"),
            "。未來的結果將轉換為 JavaScript 中的已解析或已拒絕 ",
            code("Promise"),
            "。",
        ],
        ol![li![
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html",
                "spawn_local",
            ),
            " -",
        ]],
        p![
            "在目前執行緒上產生一個 ",
            code("Future<Output = ()>"),
            "。這是在 Rust 中運行 Future 的最佳方法，而不是將其發送到 JavaScript。",
        ],
        p![
            "_",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/index.html",
                "wasm-bindgen-futures 文件",
            ),
            "._",
        ],
        h3!["[`spawn_local`](https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html)"],
        p![
            code("spawn_local"),
            " 將是 Yew 中 ",
            code("wasm-bindgen-futures"),
            " crate 中最常用的部分，因為這有助於使用具有非同步 API 的函式庫。",
        ],
        code_block(
            "rust",
            r#"use web_sys::console;
use wasm_bindgen_futures::spawn_local;

async fn my_async_fn() -> String { String::from("Hello") }

spawn_local(async {
    let mut string = my_async_fn().await;
    string.push_str(", world!");
    // 列印 "Hello, world!"
    console::log_1(&string.into());
});"#,
        ),
        p![
            "Yew 還在某些 API 中添加了對 futures 的支持，最值得注意的是您可以創建一個接受 ",
            code("async"),
            " 區塊的 ",
            code("callback_future"),
            " - 這在內部使用了 ",
            code("spawn_local"),
            "。",
        ],
        p![
            "_",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen_futures/fn.spawn_local.html",
                "spawn_local 文件",
            ),
            "._",
        ],
    ])
);
