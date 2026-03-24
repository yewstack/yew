crate::doc_page!(
    "事件",
    "/zh-Hant/docs/concepts/html/events",
    Content::new(vec![
        h2![text("介紹")],
        p![
            text("Yew 與 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/",
                text("web-sys"),
            ),
            text(" crate 集成，並使用該 crate 中的事件。下面的"),
            link!("#event-types", text("表格")),
            text("列出了在 "),
            code("html!"),
            text(" 巨集中接受的所有 "),
            code("web-sys"),
            text(" 事件。"),
        ],
        p![
            text("您仍然可以為下表中未列出的事件新增 "),
            link!("", text("Callback")),
            text("，請參閱"),
            link!(
                "#manual-event-listener",
                text("手動事件監聽器"),
            ),
            text("。"),
        ],
        h2![text("事件類型")],
        admonition![
            AdmonitionType::Tip,
            None,
            p![
                text("所有的事件類型都在 "),
                code("yew::events"),
                text(" 下重新匯出。\n使用 "),
                code("yew::events"),
                text(" 中的類型比手動將 "),
                code("web-sys"),
                text(
                    " 作為依賴項包含在您的 crate \
                     中更容易確保版本相容性，\n因為您不會使用與 Yew 指定的版本衝突的版本。",
                ),
            ],
        ],
        p![
            text("事件監聽器的名稱是在 "),
            code("html"),
            text(" 巨集中新增事件 "),
            code("Callback"),
            text(" 時預期的名稱："),
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
            text("事件名稱是監聽器名稱去掉 \"on\" 前綴，因此 "),
            code("onclick"),
            text(" 事件監聽器監聽 "),
            code("click"),
            text(" 事件。查看本頁末的"),
            link!(
                "#event-types",
                text("完整事件清單"),
            ),
            text("及其類型。"),
        ],
        h2_id!("event-bubbling", text("事件捕獲")),
        p![
            text("Yew 調度的事件遵循虛擬 DOM \
                 層次結構，向上冒泡到監聽器。目前，僅支援監聽器的冒泡階段。請注意，虛擬 DOM \
                 層次結構通常（但並非總是）與實際 DOM 層次結構相同。在處理"),
            link!("", text("傳送門")),
            text("和其他更高級技術時，這一區別很重要。對於良好實現的元件，直覺應該是事件從子元件冒泡到父元件。這樣，您在 "),
            code("html!"),
            text(
                " 中所寫的層次結構就是事件處理程序觀察到的層次結構。",
            ),
        ],
        p![text(
            "如果您不想要事件冒泡，可以透過呼叫",
        )],
        code_block("rust", r#"yew::set_event_bubbling(false);"#),
        p![text(
            "在啟動應用程式*之前*。這會加快事件處理速度，但某些元件可能會因未收到預期的事件而中斷。請謹慎使用！",
        )],
        h2![text("事件委託")],
        p![text(
            "可能會讓人驚訝的是，事件監聽器並不是直接註冊在它們被渲染的元素上。相反，事件是從 Yew \
             應用的子樹根節點委託的。不過，事件仍然以其原生形式傳遞，並且不會創建任何合成形式。這可能會導致 \
             HTML 監聽器中預期的事件與 Yew 中出現的事件之間的不符。",
        )],
        ul![
            li![
                link!(
                    "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.current_target",
                    code("Event::current_target"),
                ),
                text(
                    " 指向 Yew 子樹根節點，而不是新增監聽器的元素。如果您想存取底層的 ",
                ),
                code("HtmlElement"),
                text("，請使用 "),
                link!("", code("NodeRef")),
                text("。"),
            ],
            li_blocks![p![
                link!(
                    "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.event_phase",
                    code("Event::event_phase"),
                ),
                text(" 總是 "),
                code("Event::CAPTURING_PHASE"),
                text(
                    "。在內部，事件將表現得像是在冒泡階段，事件傳播將被重播，並且事件會",
                ),
                link!("#event-bubbling", text("向上冒泡")),
                text(
                    "，即虛擬DOM 中較高的事件監聽器將在較低的事件監聽器之後觸發。目前，Yew 不支援捕獲監聽器。\
                     這也意味著由 Yew 註冊的事件通常會在其他事件監聽器之前觸發。",
                ),
            ]],
        ],
        h2![text("具備類型的事件目標")],
        admonition![
            AdmonitionType::Caution,
            None,
            p![
                text("在本節中，"),
                bold![text("target ([`Event.target`](https://developer.mozilla.org/en-US/docs/Web/API/Event/target))")],
                text(" 總是指的是事件從其派發的元素。"),
            ],
            p![
                text("這"),
                bold![text("不一定")],
                text("總是指 "),
                code("Callback"),
                text(" 所放置的元素。"),
            ],
        ],
        p![
            text("在事件 "),
            code("Callback"),
            text(" 中，您可能想要取得該事件的目標。例如，"),
            code("change"),
            text(
                " 事件沒有提供任何訊息，但用於通知某些內容已更改。",
            ),
        ],
        p![
            text("在 Yew 中，以正確的類型獲取目標元素可以透過幾種方式完成，我們將在這裡逐一介紹。呼叫事件上的"),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.target",
                text("web_sys::Event::target"),
            ),
            text(" 傳回一個可選的"),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.EventTarget.html",
                text(" web_sys::EventTarget"),
            ),
            text(
                " 類型，當您想知道輸入元素的值時，這可能看起來不太有用。",
            ),
        ],
        p![text(
            "在下面的所有方法中，我們將解決相同的問題，以便清楚地了解方法的不同之處，而不是手邊的問題。",
        )],
        p![bold![text("問題：")]],
        p![
            text("我們在 "),
            code("<input>"),
            text(" 元素上有一個 "),
            code("onchange"),
            text(" "),
            code("Callback"),
            text("，每次呼叫時，我們希望向元件發送一個"),
            link!("components#update", text("更新")),
            text(" "),
            code("Msg"),
            text("。"),
        ],
        p![
            text("我們的 "),
            code("Msg"),
            text(" 列舉如下："),
        ],
        code_block(
            "rust",
            r#"pub enum Msg {
    InputValue(String),
}"#,
        ),
        h3![text("使用 `JsCast`")],
        p![
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html",
                text("wasm-bindgen"),
            ),
            text(" crate 有一個有用的trait："),
            link!(
                "https://rustwasm.github .io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html",
                text("JsCast"),
            ),
            text("，它允許我們在類型之間直接轉換，只要它實現了"),
            code("JsCast"),
            text(" 就行。我們可以謹慎地轉換，這涉及運行時檢查和處理 "),
            code("Option"),
            text(" 和 "),
            code("Result"),
            text(" 的邏輯，或者我們也可以冒險直接強行轉換。"),
        ],
        p![text("多說無益，看代碼：")],
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
            text(" 提供的方法是 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into",
                text("dyn_into"),
            ),
            text(" 和 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.unchecked_into",
                text("unchecked_into"),
            ),
            text("。 如你所見，它們允許我們從 "),
            code("EventTarget"),
            text(" 轉換為 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.HtmlInputElement.html",
                text("HtmlInputElement"),
            ),
            text("。 "),
            code("dyn_into"),
            text(" 方法是謹慎的，因為它會在運行時檢查類型是否實際為 "),
            code("HtmlInputElement"),
            text("，如果不是則返回 "),
            code("Err(JsValue)"),
            text("。 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html",
                text("JsValue"),
            ),
            text(
                " 是一個通用類型，將原來的物件回傳給你，以便再次嘗試轉換為別的類型。",
            ),
        ],
        p![
            text("這會兒你可能會想，什麼時候可以使用危險版本？在上面的情況下，它是安全的"),
            sup![text("1")],
            text("，因為我們將 "),
            code("Callback"),
            text(
                " 設定在一個沒有子元素的元素上，所以目標只能是同一個元素。",
            ),
        ],
        p![italic![
            sup![text("1")],
            text(" 只要牽涉到 JS 領域，就是安全的。"),
        ]],
        h3![text("使用 `TargetCast`")],
        p![bold![
            text("強烈建議先閱讀"),
            link!("#using-jscast", text("使用 JsCast")),
            text("！"),
        ]],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                code("TargetCast"),
                text(" 的設計目的是讓新用戶了解 "),
                code("JsCast"),
                text(" 的行為，但範圍更小，僅涉及事件及其目標。"),
            ],
            p![
                text("選用 "),
                code("TargetCast"),
                text(" 或 "),
                code("JsCast"),
                text(" 純粹是個人偏好，實際您會發現 "),
                code("TargetCast"),
                text(" 的實作和 "),
                code("JsCast"),
                text(" 的功能很相似。"),
            ],
        ],
        p![
            code("TargetCast"),
            text(" trait 是在 "),
            code("JsCast"),
            text(
                " 基礎之上建構的，專門用於從事件中取得類型化的事件目標。",
            ),
        ],
        p![
            code("TargetCast"),
            text(" 是 Yew 的一部分，因此無需添加依賴項即可在事件上使用 trait 方法，但它的工作方式與 "),
            code("JsCast"),
            text(" 非常相似。"),
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
            text("如果您已經了解了 "),
            code("JsCast"),
            text("，或者了解了這個 trait，您可能會發現 "),
            code("TargetCast::target_dyn_into"),
            text(" 與 "),
            code("JsCast::dyn_into"),
            text(" 相似，但專門用於事件的目標。 "),
            code("TargetCast::target_unchecked_into"),
            text(" 與 "),
            code("JsCast::unchecked_into"),
            text(" 類似，因此上面關於 "),
            code("JsCast"),
            text(" 的所有警告都適用於 "),
            code("TargetCast"),
            text("。"),
        ],
        h3![text("使用 `NodeRef`")],
        p![
            link!("", text("NodeRef")),
            text(" 可以取代查詢給定給 "),
            code("Callback"),
            text(" 的事件。"),
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
            text("透過 "),
            code("NodeRef"),
            text("，你可以忽略事件並使用 "),
            code("NodeRef::cast"),
            text(" 方法取得一個"),
            code("Option<HtmlInputElement>"),
            text(" - 這是可選的，因為在設定 "),
            code("NodeRef"),
            text(" 之前呼叫 "),
            code("cast"),
            text("，或者當類型不符時將會回傳 "),
            code("None"),
            text("。"),
        ],
        p![
            text("你可能會看到，透過使用 "),
            code("NodeRef"),
            text("，我們不必將 "),
            code("String"),
            text(" 傳回狀態，因為我們總是存取 "),
            code("input_node_ref"),
            text(" - 因此我們可以這樣做："),
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
        p![text(
            "您選擇哪種方法取決於您的元件和您的偏好，沒有所謂的*推薦*方法。",
        )],
        h2![text("手動事件監聽器")],
        p![
            text("您可能想要監聽 Yew 的 "),
            code("html"),
            text(" 巨集不支援的事件，請查看"),
            link!(
                "#event-types",
                text("這裡列出的支援的事件"),
            ),
            text("。"),
        ],
        p![
            text("為了手動為某個元素新增事件監聽器，我們需要藉助 "),
            link!("", text("NodeRef")),
            text("，以便在 "),
            code("use_effect_with"),
            text(" 中使用 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/index.html",
                text("web-sys"),
            ),
            text(" 和 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html",
                text("wasm-bindgen"),
            ),
            text(" API 新增監聽器。"),
        ],
        p![
            text("以下範例將展示如何為虛構的 "),
            code("custard"),
            text(" 事件新增監聽器。所有不受 yew 支援的事件或自訂事件都可以表示為 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html",
                text("web_sys::Event"),
            ),
            text("。如果您需要存取自訂/不受支援事件的特定方法或字段，可以使用 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html",
                text("JsCast"),
            ),
            text(" 的方法將其轉換為所需的類型。"),
        ],
        h3![text("使用 `Closure`（冗長版本）")],
        p![
            text("直接使用 "),
            code("web-sys"),
            text(" 和 "),
            code("wasm-bindgen"),
            text(" 的介面可能有點痛苦…所以要有點心理準備（"),
            link!(
                "#using-gloo-concise",
                text("感謝 gloo，有了更簡潔的方法"),
            ),
            text("）。"),
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
            text("有關 "),
            code("Closure"),
            text(" 的更多信息，請參見 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/examples/closures.html",
                text("wasm-bindgen 指南"),
            ),
            text("。"),
        ],
        h3![text("使用 `gloo`（簡潔版本）")],
        p![
            text("更方便的方法是使用 "),
            code("gloo"),
            text("，更具體地說是 "),
            link!(
                "https://docs.rs/gloo-events/0.1.1/gloo_events/index.html",
                text("gloo_events"),
            ),
            text("， 它是 "),
            code("web-sys"),
            text("、"),
            code("wasm-bindgen"),
            text(" 的高層抽象實作。"),
        ],
        p![
            code("gloo_events"),
            text(" 提供了 "),
            code("EventListener"),
            text(" 類型，可以用來建立和儲存事件監聽器。"),
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
            text("有關 "),
            code("EventListener"),
            text(" 的更多信息，請參見 "),
            link!(
                "https://docs.rs/gloo-events/0.1.1/gloo_events/struct.EventListener.html",
                text("gloo_events docs.rs"),
            ),
            text("。"),
        ],
        h2_id!("available-events", text("可用事件的完整清單")),
        p![
            text("| 偵聽器名稱                  | "),
            code("web_sys"),
            text(" 事件類型                                                                    | | --------------------------- | ------------------------------------------------------------------------------------- | | "),
            code("onabort"),
            text("                   | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onauxclick"),
            text("                | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent")),
            text("           | | "),
            code("onblur"),
            text("                    | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", text("FocusEvent")),
            text("           | | "),
            code("oncancel"),
            text("                  | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("oncanplay"),
            text("                 | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("oncanplaythrough"),
            text("          | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onchange"),
            text("                  | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onclick"),
            text("                   | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent")),
            text("           | | "),
            code("onclose"),
            text("                   | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("oncontextmenu"),
            text("             | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent")),
            text("           | | "),
            code("oncuechange"),
            text("               | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("ondblclick"),
            text("                | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent")),
            text("           | | "),
            code("ondrag"),
            text("                    | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", text("DragEvent")),
            text("             | | "),
            code("ondragend"),
            text("                 | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", text("DragEvent")),
            text("             | | "),
            code("ondragenter"),
            text("               | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", text("DragEvent")),
            text("             | | "),
            code("ondragexit"),
            text("                | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", text("DragEvent")),
            text("             | | "),
            code("ondragleave"),
            text("               | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", text("DragEvent")),
            text("             | | "),
            code("ondragover"),
            text("                | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", text("DragEvent")),
            text("             | | "),
            code("ondragstart"),
            text("               | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", text("DragEvent")),
            text("             | | "),
            code("ondrop"),
            text("                    | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", text("DragEvent")),
            text("             | | "),
            code("ondurationchange"),
            text("          | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onemptied"),
            text("                 | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onended"),
            text("                   | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onerror"),
            text("                   | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onfocus"),
            text("                   | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", text("FocusEvent")),
            text("           | | "),
            code("onfocusin"),
            text("                 | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", text("FocusEvent")),
            text("           | | "),
            code("onfocusout"),
            text("                | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", text("FocusEvent")),
            text("           | | "),
            code("onformdata"),
            text("                | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("oninput"),
            text("                   | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.InputEvent.html", text("InputEvent")),
            text("           | | "),
            code("oninvalid"),
            text("                 | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onkeydown"),
            text("                 | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html", text("KeyboardEvent")),
            text("     | | "),
            code("onkeypress"),
            text("                | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html", text("KeyboardEvent")),
            text("     | | "),
            code("onkeyup"),
            text("                   | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html", text("KeyboardEvent")),
            text("     | | "),
            code("onload"),
            text("                    | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onloadeddata"),
            text("              | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onloadedmetadata"),
            text("          | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onloadstart"),
            text("               | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html", text("ProgressEvent")),
            text("     | | "),
            code("onmousedown"),
            text("               | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent")),
            text("           | | "),
            code("onmouseenter"),
            text("              | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent")),
            text("           | | "),
            code("onmouseleave"),
            text("              | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent")),
            text("           | | "),
            code("onmousemove"),
            text("               | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent")),
            text("           | | "),
            code("onmouseout"),
            text("                | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent")),
            text("           | | "),
            code("onmouseover"),
            text("               | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent")),
            text("           | | "),
            code("onmouseup"),
            text("                 | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent")),
            text("           | | "),
            code("onpause"),
            text("                   | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onplay"),
            text("                    | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onplaying"),
            text("                 | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onprogress"),
            text("                | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html", text("ProgressEvent")),
            text("     | | "),
            code("onratechange"),
            text("              | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onreset"),
            text("                   | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onresize"),
            text("                  | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onscroll"),
            text("                  | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onsecuritypolicyviolation"),
            text(" | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onseeked"),
            text("                  | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onseeking"),
            text("                 | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onselect"),
            text("                  | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onslotchange"),
            text("              | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onstalled"),
            text("                 | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onsubmit"),
            text("                  | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.SubmitEvent.html", text("SubmitEvent")),
            text("         | | "),
            code("onsuspend"),
            text("                 | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("ontimeupdate"),
            text("              | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("ontoggle"),
            text("                  | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onvolumechange"),
            text("            | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onwaiting"),
            text("                 | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onwheel"),
            text("                   | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.WheelEvent.html", text("WheelEvent")),
            text("           | | "),
            code("oncopy"),
            text("                    | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("oncut"),
            text("                     | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onpaste"),
            text("                   | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onanimationcancel"),
            text("         | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", text("AnimationEvent")),
            text("   | | "),
            code("onanimationend"),
            text("            | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", text("AnimationEvent")),
            text("   | | "),
            code("onanimationiteration"),
            text("      | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", text("AnimationEvent")),
            text("   | | "),
            code("onanimationstart"),
            text("          | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", text("AnimationEvent")),
            text("   | | "),
            code("ongotpointercapture"),
            text("       | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent")),
            text("       | | "),
            code("onloadend"),
            text("                 | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html", text("ProgressEvent")),
            text("     | | "),
            code("onlostpointercapture"),
            text("      | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent")),
            text("       | | "),
            code("onpointercancel"),
            text("           | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent")),
            text("       | | "),
            code("onpointerdown"),
            text("             | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent")),
            text("       | | "),
            code("onpointerenter"),
            text("            | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent")),
            text("       | | "),
            code("onpointerleave"),
            text("            | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent")),
            text("       | | "),
            code("onpointerlockchange"),
            text("       | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onpointerlockerror"),
            text("        | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onpointermove"),
            text("             | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent")),
            text("       | | "),
            code("onpointerout"),
            text("              | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent")),
            text("       | | "),
            code("onpointerover"),
            text("             | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent")),
            text("       | | "),
            code("onpointerup"),
            text("               | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent")),
            text("       | | "),
            code("onselectionchange"),
            text("         | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onselectstart"),
            text("             | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("onshow"),
            text("                    | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event")),
            text("                     | | "),
            code("ontouchcancel"),
            text("             | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", text("TouchEvent")),
            text("           | | "),
            code("ontouchend"),
            text("                | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", text("TouchEvent")),
            text("           | | "),
            code("ontouchmove"),
            text("               | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", text("TouchEvent")),
            text("           | | "),
            code("ontouchstart"),
            text("              | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", text("TouchEvent")),
            text("           | | "),
            code("ontransitioncancel"),
            text("        | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", text("TransitionEvent")),
            text(" | | "),
            code("ontransitionend"),
            text("           | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", text("TransitionEvent")),
            text(" | | "),
            code("ontransitionrun"),
            text("           | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", text("TransitionEvent")),
            text(" | | "),
            code("ontransitionstart"),
            text("         | "),
            link!("https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", text("TransitionEvent")),
            text(" |"),
        ],
    ])
);
