crate::doc_page!(
    "事件",
    "/zh-Hant/docs/concepts/html/events",
    Content::new(vec![
        h2!["介紹"],
        p![
            "Yew 與 ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/",
                "web-sys",
            ),
            " crate 集成，並使用該 crate 中的事件。下面的",
            link!("#event-types", "表格"),
            "列出了在 ",
            code("html!"),
            " 巨集中接受的所有 ",
            code("web-sys"),
            " 事件。",
        ],
        p![
            "您仍然可以為下表中未列出的事件新增 ",
            link!("", "Callback"),
            "，請參閱",
            link!(
                "#manual-event-listener",
                "手動事件監聽器",
            ),
            "。",
        ],
        h2!["事件類型"],
        admonition![
            AdmonitionType::Tip,
            None,
            p![
                "所有的事件類型都在 ",
                code("yew::events"),
                " 下重新匯出。\n使用 ",
                code("yew::events"),
                " 中的類型比手動將 ",
                code("web-sys"),
                " 作為依賴項包含在您的 crate \
                     中更容易確保版本相容性，\n因為您不會使用與 Yew 指定的版本衝突的版本。",
            ],
        ],
        p![
            "事件監聽器的名稱是在 ",
            code("html"),
            " 巨集中新增事件 ",
            code("Callback"),
            " 時預期的名稱：",
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

html! {
    <button onclick={Callback::from(|_| ())}>
    //      ^^^^^^^ event listener name
        { "Click me!" }
    </button>
};"#,
        ),
        p![
            "事件名稱是監聽器名稱去掉 \"on\" 前綴，因此 ",
            code("onclick"),
            " 事件監聽器監聽 ",
            code("click"),
            " 事件。查看本頁末的",
            link!(
                "#event-types",
                "完整事件清單",
            ),
            "及其類型。",
        ],
        h2_id!("event-bubbling", "事件捕獲"),
        p![
            "Yew 調度的事件遵循虛擬 DOM \
                 層次結構，向上冒泡到監聽器。目前，僅支援監聽器的冒泡階段。請注意，虛擬 DOM \
                 層次結構通常（但並非總是）與實際 DOM 層次結構相同。在處理",
            link!("", "傳送門"),
            "和其他更高級技術時，這一區別很重要。對於良好實現的元件，直覺應該是事件從子元件冒泡到父元件。這樣，您在 ",
            code("html!"),
            " 中所寫的層次結構就是事件處理程序觀察到的層次結構。",
        ],
        p!["如果您不想要事件冒泡，可以透過呼叫"],
        code_block("rust", r#"yew::set_event_bubbling(false);"#),
        p!["在啟動應用程式*之前*。這會加快事件處理速度，但某些元件可能會因未收到預期的事件而中斷。請謹慎使用！"],
        h2!["事件委託"],
        p!["可能會讓人驚訝的是，事件監聽器並不是直接註冊在它們被渲染的元素上。相反，事件是從 Yew \
             應用的子樹根節點委託的。不過，事件仍然以其原生形式傳遞，並且不會創建任何合成形式。這可能會導致 \
             HTML 監聽器中預期的事件與 Yew 中出現的事件之間的不符。"],
        ul![
            li![
                link!(
                    "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.current_target",
                    code("Event::current_target"),
                ),
                " 指向 Yew 子樹根節點，而不是新增監聽器的元素。如果您想存取底層的 ",
                code("HtmlElement"),
                "，請使用 ",
                link!("", code("NodeRef")),
                "。",
            ],
            li_blocks![p![
                link!(
                    "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.event_phase",
                    code("Event::event_phase"),
                ),
                " 總是 ",
                code("Event::CAPTURING_PHASE"),
                "。在內部，事件將表現得像是在冒泡階段，事件傳播將被重播，並且事件會",
                link!("#event-bubbling", "向上冒泡"),
                "，即虛擬DOM 中較高的事件監聽器將在較低的事件監聽器之後觸發。目前，Yew 不支援捕獲監聽器。\
                     這也意味著由 Yew 註冊的事件通常會在其他事件監聽器之前觸發。",
            ]],
        ],
        h2!["具備類型的事件目標"],
        admonition![
            AdmonitionType::Caution,
            None,
            p![
                "在本節中，",
                bold!["target ([`Event.target`](https://developer.mozilla.org/en-US/docs/Web/API/Event/target))"],
                " 總是指的是事件從其派發的元素。",
            ],
            p![
                "這",
                bold!["不一定"],
                "總是指 ",
                code("Callback"),
                " 所放置的元素。",
            ],
        ],
        p![
            "在事件 ",
            code("Callback"),
            " 中，您可能想要取得該事件的目標。例如，",
            code("change"),
            " 事件沒有提供任何訊息，但用於通知某些內容已更改。",
        ],
        p![
            "在 Yew 中，以正確的類型獲取目標元素可以透過幾種方式完成，我們將在這裡逐一介紹。呼叫事件上的",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.target",
                "web_sys::Event::target",
            ),
            " 傳回一個可選的",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.EventTarget.html",
                " web_sys::EventTarget",
            ),
            " 類型，當您想知道輸入元素的值時，這可能看起來不太有用。",
        ],
        p!["在下面的所有方法中，我們將解決相同的問題，以便清楚地了解方法的不同之處，而不是手邊的問題。"],
        p![bold!["問題："]],
        p![
            "我們在 ",
            code("<input>"),
            " 元素上有一個 ",
            code("onchange"),
            " ",
            code("Callback"),
            "，每次呼叫時，我們希望向元件發送一個",
            link!("components#update", "更新"),
            " ",
            code("Msg"),
            "。",
        ],
        p![
            "我們的 ",
            code("Msg"),
            " 列舉如下：",
        ],
        code_block(
            "rust",
            r#"pub enum Msg {
    InputValue(String),
}"#,
        ),
        h3!["使用 `JsCast`"],
        p![
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html",
                "wasm-bindgen",
            ),
            " crate 有一個有用的trait：",
            link!(
                "https://rustwasm.github .io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html",
                "JsCast",
            ),
            "，它允許我們在類型之間直接轉換，只要它實現了",
            code("JsCast"),
            " 就行。我們可以謹慎地轉換，這涉及運行時檢查和處理 ",
            code("Option"),
            " 和 ",
            code("Result"),
            " 的邏輯，或者我們也可以冒險直接強行轉換。",
        ],
        p!["多說無益，看代碼："],
        code_block_title(
            "toml",
            "Cargo.toml",
            r#"[dependencies]
# 需要 wasm-bindgen 用於呼叫 JsCast
wasm-bindgen = "0.2""#,
        ),
        code_block(
            "rust",
            r#"//highlight-next-line
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[component]
fn MyComponent() -> Html {
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let on_cautious_change = {
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |e: Event| {
            // 當事件被建立時，目標是未定義的，只有在派發時才會新增目標。
            let target: Option<EventTarget> = e.target();
            // 事件可能會冒泡，因此此偵聽器可能會捕獲不是 HtmlInputElement 類型的子元素的事件。
            //highlight-next-line
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    let on_dangerous_change = Callback::from(move |e: Event| {
        let target: EventTarget = e
            .target()
            .expect("Event should have a target when dispatched");
        // 你必須了解 target 是 HtmlInputElement，否則呼叫 value 將是未定義行為（UB）。
        // 在這裡，我們確信這是輸入元素，因此我們可以在不檢查的情況下將其轉換為適當的類型。
        //highlight-next-line
        input_value_handle.set(target.unchecked_into::<HtmlInputElement>().value());
    });

    html! {
        <>
            <label for="cautious-input">
                { "My cautious input:" }
                <input onchange={on_cautious_change}
                    id="cautious-input"
                    type="text"
                    value={input_value.clone()}
                />
            </label>
            <label for="dangerous-input">
                { "My dangerous input:" }
                <input onchange={on_dangerous_change}
                    id="dangerous-input"
                    type="text"
                    value={input_value}
                />
            </label>
        </>
    }
}"#,
        ),
        p![
            code("JsCast"),
            " 提供的方法是 ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into",
                "dyn_into",
            ),
            " 和 ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.unchecked_into",
                "unchecked_into",
            ),
            "。 如你所見，它們允許我們從 ",
            code("EventTarget"),
            " 轉換為 ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.HtmlInputElement.html",
                "HtmlInputElement",
            ),
            "。 ",
            code("dyn_into"),
            " 方法是謹慎的，因為它會在運行時檢查類型是否實際為 ",
            code("HtmlInputElement"),
            "，如果不是則返回 ",
            code("Err(JsValue)"),
            "。 ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html",
                "JsValue",
            ),
            " 是一個通用類型，將原來的物件回傳給你，以便再次嘗試轉換為別的類型。",
        ],
        p![
            "這會兒你可能會想，什麼時候可以使用危險版本？在上面的情況下，它是安全的",
            sup!["1"],
            "，因為我們將 ",
            code("Callback"),
            " 設定在一個沒有子元素的元素上，所以目標只能是同一個元素。",
        ],
        p![italic![
            sup!["1"],
            " 只要牽涉到 JS 領域，就是安全的。",
        ]],
        h3!["使用 `TargetCast`"],
        p![bold![
            "強烈建議先閱讀",
            link!("#using-jscast", "使用 JsCast"),
            "！",
        ]],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                code("TargetCast"),
                " 的設計目的是讓新用戶了解 ",
                code("JsCast"),
                " 的行為，但範圍更小，僅涉及事件及其目標。",
            ],
            p![
                "選用 ",
                code("TargetCast"),
                " 或 ",
                code("JsCast"),
                " 純粹是個人偏好，實際您會發現 ",
                code("TargetCast"),
                " 的實作和 ",
                code("JsCast"),
                " 的功能很相似。",
            ],
        ],
        p![
            code("TargetCast"),
            " trait 是在 ",
            code("JsCast"),
            " 基礎之上建構的，專門用於從事件中取得類型化的事件目標。",
        ],
        p![
            code("TargetCast"),
            " 是 Yew 的一部分，因此無需添加依賴項即可在事件上使用 trait 方法，但它的工作方式與 ",
            code("JsCast"),
            " 非常相似。",
        ],
        code_block(
            "rust",
            r#"use web_sys::HtmlInputElement;
use yew::prelude::*;

#[component]
fn MyComponent() -> Html {
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let on_cautious_change = {
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    let on_dangerous_change = Callback::from(move |e: Event| {
        // 你必須清楚 target 是 HtmlInputElement，否則呼叫 value 將是未定義行為（UB）。
        //highlight-next-line
        input_value_handle.set(e.target_unchecked_into::<HtmlInputElement>().value());
    });

    html! {
        <>
            <label for="cautious-input">
                { "My cautious input:" }
                <input onchange={on_cautious_change}
                    id="cautious-input"
                    type="text"
                    value={input_value.clone()}
                />
            </label>
            <label for="dangerous-input">
                { "My dangerous input:" }
                <input onchange={on_dangerous_change}
                    id="dangerous-input"
                    type="text"
                    value={input_value}
                />
            </label>
        </>
    }
}"#,
        ),
        p![
            "如果您已經了解了 ",
            code("JsCast"),
            "，或者了解了這個 trait，您可能會發現 ",
            code("TargetCast::target_dyn_into"),
            " 與 ",
            code("JsCast::dyn_into"),
            " 相似，但專門用於事件的目標。 ",
            code("TargetCast::target_unchecked_into"),
            " 與 ",
            code("JsCast::unchecked_into"),
            " 類似，因此上面關於 ",
            code("JsCast"),
            " 的所有警告都適用於 ",
            code("TargetCast"),
            "。",
        ],
        h3!["使用 `NodeRef`"],
        p![
            link!("", "NodeRef"),
            " 可以取代查詢給定給 ",
            code("Callback"),
            " 的事件。",
        ],
        code_block(
            "rust",
            r#"use web_sys::HtmlInputElement;
use yew::prelude::*;

#[component]
fn MyComponent() -> Html {
    //highlight-next-line
    let input_node_ref = use_node_ref();

    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let onchange = {
        let input_node_ref = input_node_ref.clone();

        Callback::from(move |_| {
            //highlight-next-line
            let input = input_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    html! {
        <>
            <label for="my-input">
                { "My input:" }
                //highlight-next-line
                <input ref={input_node_ref}
                    {onchange}
                    id="my-input"
                    type="text"
                    value={input_value}
                />
            </label>
        </>
    }
}"#,
        ),
        p![
            "透過 ",
            code("NodeRef"),
            "，你可以忽略事件並使用 ",
            code("NodeRef::cast"),
            " 方法取得一個",
            code("Option<HtmlInputElement>"),
            " - 這是可選的，因為在設定 ",
            code("NodeRef"),
            " 之前呼叫 ",
            code("cast"),
            "，或者當類型不符時將會回傳 ",
            code("None"),
            "。",
        ],
        p![
            "你可能會看到，透過使用 ",
            code("NodeRef"),
            "，我們不必將 ",
            code("String"),
            " 傳回狀態，因為我們總是存取 ",
            code("input_node_ref"),
            " - 因此我們可以這樣做：",
        ],
        code_block(
            "rust",
            r#"use web_sys::HtmlInputElement;
use yew::prelude::*;

#[component]
fn MyComponent() -> Html {
    let input_node_ref = use_node_ref();

    //highlight-start
    let onchange = {
        let input_node_ref = input_node_ref.clone();

        Callback::from(move |_| {
            if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                // 對 value 做點什麼
            }
        })
    };
    //highlight-end

    html! {
        <>
            <label for="my-input">
                { "My input:" }
                <input ref={input_node_ref}
                    {onchange}
                    id="my-input"
                    type="text"
                />
            </label>
        </>
    }
}"#,
        ),
        p!["您選擇哪種方法取決於您的元件和您的偏好，沒有所謂的*推薦*方法。"],
        h2!["手動事件監聽器"],
        p![
            "您可能想要監聽 Yew 的 ",
            code("html"),
            " 巨集不支援的事件，請查看",
            link!(
                "#event-types",
                "這裡列出的支援的事件",
            ),
            "。",
        ],
        p![
            "為了手動為某個元素新增事件監聽器，我們需要藉助 ",
            link!("", "NodeRef"),
            "，以便在 ",
            code("use_effect_with"),
            " 中使用 ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/index.html",
                "web-sys",
            ),
            " 和 ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html",
                "wasm-bindgen",
            ),
            " API 新增監聽器。",
        ],
        p![
            "以下範例將展示如何為虛構的 ",
            code("custard"),
            " 事件新增監聽器。所有不受 yew 支援的事件或自訂事件都可以表示為 ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html",
                "web_sys::Event",
            ),
            "。如果您需要存取自訂/不受支援事件的特定方法或字段，可以使用 ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html",
                "JsCast",
            ),
            " 的方法將其轉換為所需的類型。",
        ],
        h3!["使用 `Closure`（冗長版本）"],
        p![
            "直接使用 ",
            code("web-sys"),
            " 和 ",
            code("wasm-bindgen"),
            " 的介面可能有點痛苦…所以要有點心理準備（",
            link!(
                "#using-gloo-concise",
                "感謝 gloo，有了更簡潔的方法",
            ),
            "）。",
        ],
        code_block(
            "rust",
            r#"use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlElement;
use yew::prelude::*;

#[component]
fn MyComponent() -> Html {
    let div_node_ref = use_node_ref();

    use_effect_with(
        div_node_ref.clone(),
        {
            let div_node_ref = div_node_ref.clone();

            move |_| {
                let mut custard_listener = None;

                if let Some(element) = div_node_ref.cast::<HtmlElement>() {
                    // 建立您通常會建立的 Callback
                    let oncustard = Callback::from(move |_: Event| {
                        // 對 custard 做點什麼..
                    });

                    // 從 Box<dyn Fn> 創建一個 Closure - 這必須是 'static
                    let listener =
                        Closure::<dyn Fn(Event)>::wrap(
                            Box::new(move |e: Event| oncustard.emit(e))
                        );

                    element
                        .add_event_listener_with_callback(
                            "custard",
                            listener.as_ref().unchecked_ref()
                        )
                        .unwrap();

                    custard_listener = Some(listener);
                }

                move || drop(custard_listener)
            }
        }
    );

    html! {
        <div ref={div_node_ref} id="my-div"></div>
    }
}"#,
        ),
        p![
            "有關 ",
            code("Closure"),
            " 的更多信息，請參見 ",
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/examples/closures.html",
                "wasm-bindgen 指南",
            ),
            "。",
        ],
        h3!["使用 `gloo`（簡潔版本）"],
        p![
            "更方便的方法是使用 ",
            code("gloo"),
            "，更具體地說是 ",
            link!(
                "https://docs.rs/gloo-events/0.1.1/gloo_events/index.html",
                "gloo_events",
            ),
            "， 它是 ",
            code("web-sys"),
            "、",
            code("wasm-bindgen"),
            " 的高層抽象實作。",
        ],
        p![
            code("gloo_events"),
            " 提供了 ",
            code("EventListener"),
            " 類型，可以用來建立和儲存事件監聽器。",
        ],
        code_block_title(
            "toml",
            "Cargo.toml",
            r#"[dependencies]
gloo-events = "0.1""#,
        ),
        code_block(
            "rust",
            r#"use web_sys::HtmlElement;
use yew::prelude::*;

use gloo::events::EventListener;

#[component]
fn MyComponent() -> Html {
    let div_node_ref = use_node_ref();

    use_effect_with(
        div_node_ref.clone(),
        {
            let div_node_ref = div_node_ref.clone();

            move |_| {
                let mut custard_listener = None;

                if let Some(element) = div_node_ref.cast::<HtmlElement>() {
                    // 建立您通常會建立的 Callback
                    let oncustard = Callback::from(move |_: Event| {
                        // 對 custard 做點什麼..
                    });

                    // 從 Box<dyn Fn> 創建一個 Closure - 這必須是 'static
                    let listener = EventListener::new(
                        &element,
                        "custard",
                        move |e| oncustard.emit(e.clone())
                    );

                    custard_listener = Some(listener);
                }

                move || drop(custard_listener)
            }
        }
    );

    html! {
        <div ref={div_node_ref} id="my-div"></div>
    }
}"#,
        ),
        p![
            "有關 ",
            code("EventListener"),
            " 的更多信息，請參見 ",
            link!(
                "https://docs.rs/gloo-events/0.1.1/gloo_events/struct.EventListener.html",
                "gloo_events docs.rs",
            ),
            "。",
        ],
        h2_id!("available-events", "可用事件的完整清單"),
        p![
            "| 偵聽器名稱                  | ",
            code("web_sys"),
            " 事件類型                                                                    | | --------------------------- | ------------------------------------------------------------------------------------- | | ",
            code("onabort"),
            "                   | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onauxclick"),
            "                | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"),
            "           | | ",
            code("onblur"),
            "                    | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", "FocusEvent"),
            "           | | ",
            code("oncancel"),
            "                  | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("oncanplay"),
            "                 | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("oncanplaythrough"),
            "          | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onchange"),
            "                  | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onclick"),
            "                   | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"),
            "           | | ",
            code("onclose"),
            "                   | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("oncontextmenu"),
            "             | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"),
            "           | | ",
            code("oncuechange"),
            "               | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("ondblclick"),
            "                | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"),
            "           | | ",
            code("ondrag"),
            "                    | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"),
            "             | | ",
            code("ondragend"),
            "                 | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"),
            "             | | ",
            code("ondragenter"),
            "               | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"),
            "             | | ",
            code("ondragexit"),
            "                | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"),
            "             | | ",
            code("ondragleave"),
            "               | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"),
            "             | | ",
            code("ondragover"),
            "                | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"),
            "             | | ",
            code("ondragstart"),
            "               | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"),
            "             | | ",
            code("ondrop"),
            "                    | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"),
            "             | | ",
            code("ondurationchange"),
            "          | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onemptied"),
            "                 | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onended"),
            "                   | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onerror"),
            "                   | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onfocus"),
            "                   | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", "FocusEvent"),
            "           | | ",
            code("onfocusin"),
            "                 | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", "FocusEvent"),
            "           | | ",
            code("onfocusout"),
            "                | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", "FocusEvent"),
            "           | | ",
            code("onformdata"),
            "                | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("oninput"),
            "                   | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.InputEvent.html", "InputEvent"),
            "           | | ",
            code("oninvalid"),
            "                 | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onkeydown"),
            "                 | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html", "KeyboardEvent"),
            "     | | ",
            code("onkeypress"),
            "                | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html", "KeyboardEvent"),
            "     | | ",
            code("onkeyup"),
            "                   | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html", "KeyboardEvent"),
            "     | | ",
            code("onload"),
            "                    | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onloadeddata"),
            "              | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onloadedmetadata"),
            "          | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onloadstart"),
            "               | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html", "ProgressEvent"),
            "     | | ",
            code("onmousedown"),
            "               | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"),
            "           | | ",
            code("onmouseenter"),
            "              | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"),
            "           | | ",
            code("onmouseleave"),
            "              | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"),
            "           | | ",
            code("onmousemove"),
            "               | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"),
            "           | | ",
            code("onmouseout"),
            "                | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"),
            "           | | ",
            code("onmouseover"),
            "               | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"),
            "           | | ",
            code("onmouseup"),
            "                 | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"),
            "           | | ",
            code("onpause"),
            "                   | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onplay"),
            "                    | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onplaying"),
            "                 | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onprogress"),
            "                | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html", "ProgressEvent"),
            "     | | ",
            code("onratechange"),
            "              | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onreset"),
            "                   | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onresize"),
            "                  | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onscroll"),
            "                  | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onsecuritypolicyviolation"),
            " | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onseeked"),
            "                  | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onseeking"),
            "                 | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onselect"),
            "                  | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onslotchange"),
            "              | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onstalled"),
            "                 | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onsubmit"),
            "                  | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.SubmitEvent.html", "SubmitEvent"),
            "         | | ",
            code("onsuspend"),
            "                 | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("ontimeupdate"),
            "              | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("ontoggle"),
            "                  | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onvolumechange"),
            "            | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onwaiting"),
            "                 | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onwheel"),
            "                   | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.WheelEvent.html", "WheelEvent"),
            "           | | ",
            code("oncopy"),
            "                    | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("oncut"),
            "                     | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onpaste"),
            "                   | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onanimationcancel"),
            "         | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", "AnimationEvent"),
            "   | | ",
            code("onanimationend"),
            "            | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", "AnimationEvent"),
            "   | | ",
            code("onanimationiteration"),
            "      | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", "AnimationEvent"),
            "   | | ",
            code("onanimationstart"),
            "          | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", "AnimationEvent"),
            "   | | ",
            code("ongotpointercapture"),
            "       | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"),
            "       | | ",
            code("onloadend"),
            "                 | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html", "ProgressEvent"),
            "     | | ",
            code("onlostpointercapture"),
            "      | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"),
            "       | | ",
            code("onpointercancel"),
            "           | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"),
            "       | | ",
            code("onpointerdown"),
            "             | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"),
            "       | | ",
            code("onpointerenter"),
            "            | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"),
            "       | | ",
            code("onpointerleave"),
            "            | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"),
            "       | | ",
            code("onpointerlockchange"),
            "       | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onpointerlockerror"),
            "        | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onpointermove"),
            "             | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"),
            "       | | ",
            code("onpointerout"),
            "              | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"),
            "       | | ",
            code("onpointerover"),
            "             | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"),
            "       | | ",
            code("onpointerup"),
            "               | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"),
            "       | | ",
            code("onselectionchange"),
            "         | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onselectstart"),
            "             | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("onshow"),
            "                    | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"),
            "                     | | ",
            code("ontouchcancel"),
            "             | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", "TouchEvent"),
            "           | | ",
            code("ontouchend"),
            "                | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", "TouchEvent"),
            "           | | ",
            code("ontouchmove"),
            "               | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", "TouchEvent"),
            "           | | ",
            code("ontouchstart"),
            "              | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", "TouchEvent"),
            "           | | ",
            code("ontransitioncancel"),
            "        | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", "TransitionEvent"),
            " | | ",
            code("ontransitionend"),
            "           | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", "TransitionEvent"),
            " | | ",
            code("ontransitionrun"),
            "           | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", "TransitionEvent"),
            " | | ",
            code("ontransitionstart"),
            "         | ",
            link!("https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", "TransitionEvent"),
            " |",
        ],
    ])
);
