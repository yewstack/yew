pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("介紹")]),
        p(vec![
            text("Yew 與 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/",
                vec![code("web-sys")],
            ),
            text(" crate 集成，並使用該 crate 中的事件。下面的"),
            link("#event-types", vec![text("表格")]),
            text("列出了在 "),
            code("html!"),
            text(" 巨集中接受的所有 "),
            code("web-sys"),
            text(" 事件。"),
        ]),
        p(vec![
            text("您仍然可以為下表中未列出的事件新增 "),
            link("/zh-Hant/docs/concepts/function-components/callbacks", vec![code("Callback")]),
            text("，請參閱"),
            link(
                "#manual-event-listener",
                vec![text("手動事件監聽器")],
            ),
            text("。"),
        ]),
        h2(vec![text("事件類型")]),
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p(vec![
                text("所有的事件類型都在 "),
                code("yew::events"),
                text(" 下重新匯出。\n使用 "),
                code("yew::events"),
                text(" 中的類型比手動將 "),
                code("web-sys"),
                text(" 作為依賴項包含在您的 crate 中更容易確保版本相容性，\n因為您不會使用與 Yew 指定的版本衝突的版本。"),
            ])],
        ),
        p(vec![
            text("事件監聽器的名稱是在 "),
            code("html"),
            text(" 巨集中新增事件 "),
            code("Callback"),
            text(" 時預期的名稱："),
        ]),
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
        p(vec![
            text("事件名稱是監聽器名稱去掉 \"on\" 前綴，因此 "),
            code("onclick"),
            text(" 事件監聽器監聽 "),
            code("click"),
            text(" 事件。查看本頁末的"),
            link("#available-events", vec![text("完整事件清單")]),
            text("及其類型。"),
        ]),
        h2(vec![text("事件捕獲")]),
        p(vec![
            text("Yew 調度的事件遵循虛擬 DOM 層次結構，向上冒泡到監聽器。目前，僅支援監聽器的冒泡階段。請注意，虛擬 DOM 層次結構通常（但並非總是）與實際 DOM 層次結構相同。在處理"),
            link("/zh-Hant/docs/advanced-topics/portals", vec![text("傳送門")]),
            text("和其他更高級技術時，這一區別很重要。對於良好實現的元件，直覺應該是事件從子元件冒泡到父元件。這樣，您在 "),
            code("html!"),
            text(" 中所寫的層次結構就是事件處理程序觀察到的層次結構。"),
        ]),
        p(vec![text(
            "如果您不想要事件冒泡，可以透過呼叫",
        )]),
        code_block("rust", r#"yew::set_event_bubbling(false);"#),
        p(vec![
            text("在啟動應用程式"),
            italic(vec![text("之前")]),
            text("。這會加快事件處理速度，但某些元件可能會因未收到預期的事件而中斷。請謹慎使用！"),
        ]),
        h2(vec![text("事件委託")]),
        p(vec![text(
            "可能會讓人驚訝的是，事件監聽器並不是直接註冊在它們被渲染的元素上。相反，事件是從 Yew 應用的子樹根節點委託的。不過，事件仍然以其原生形式傳遞，並且不會創建任何合成形式。這可能會導致 HTML 監聽器中預期的事件與 Yew 中出現的事件之間的不符。",
        )]),
        ul(vec![
            li(vec![
                link(
                    "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.current_target",
                    vec![code("Event::current_target")],
                ),
                text(" 指向 Yew 子樹根節點，而不是新增監聽器的元素。如果您想存取底層的 "),
                code("HtmlElement"),
                text("，請使用 "),
                link("/zh-Hant/docs/concepts/function-components/node-refs", vec![text("NodeRef")]),
                text("。"),
            ]),
            li(vec![
                link(
                    "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.event_phase",
                    vec![code("Event::event_phase")],
                ),
                text(" 總是 "),
                link(
                    "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#associatedconstant.CAPTURING_PHASE",
                    vec![code("Event::CAPTURING_PHASE")],
                ),
                text("。在內部，事件將表現得像是在冒泡階段，事件傳播將被重播，並且事件會"),
                link("#event-bubbling", vec![text("向上冒泡")]),
                text("，即虛擬DOM 中較高的事件監聽器將在較低的事件監聽器之後觸發。目前，Yew 不支援捕獲監聽器。"),
            ]),
        ]),
        p(vec![text(
            "這也意味著由 Yew 註冊的事件通常會在其他事件監聽器之前觸發。",
        )]),
        h2(vec![text("具備類型的事件目標")]),
        admonition(
            AdmonitionType::Caution,
            None,
            vec![
                p(vec![
                    text("在本節中，"),
                    bold(vec![
                        text("target ("),
                        link(
                            "https://developer.mozilla.org/en-US/docs/Web/API/Event/target",
                            vec![code("Event.target")],
                        ),
                        text(")"),
                    ]),
                    text(" 總是指的是事件從其派發的元素。"),
                ]),
                p(vec![
                    text("這"),
                    bold(vec![text("不一定")]),
                    text("總是指 "),
                    code("Callback"),
                    text(" 所放置的元素。"),
                ]),
            ],
        ),
        p(vec![
            text("在事件 "),
            code("Callback"),
            text(" 中，您可能想要取得該事件的目標。例如，"),
            code("change"),
            text(" 事件沒有提供任何訊息，但用於通知某些內容已更改。"),
        ]),
        p(vec![
            text("在 Yew 中，以正確的類型獲取目標元素可以透過幾種方式完成，我們將在這裡逐一介紹。呼叫事件上的"),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.target",
                vec![code("web_sys::Event::target")],
            ),
            text(" 傳回一個可選的"),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.EventTarget.html",
                vec![code("web_sys::EventTarget")],
            ),
            text(" 類型，當您想知道輸入元素的值時，這可能看起來不太有用。"),
        ]),
        p(vec![text(
            "在下面的所有方法中，我們將解決相同的問題，以便清楚地了解方法的不同之處，而不是手邊的問題。",
        )]),
        p(vec![bold(vec![text("問題：")])]),
        p(vec![
            text("我們在 "),
            code("<input>"),
            text(" 元素上有一個 "),
            code("onchange"),
            text(" "),
            code("Callback"),
            text("，每次呼叫時，我們希望向元件發送一個"),
            link("components#update", vec![text("更新")]),
            text(" "),
            code("Msg"),
            text("。"),
        ]),
        p(vec![
            text("我們的 "),
            code("Msg"),
            text(" 列舉如下："),
        ]),
        code_block(
            "rust",
            r#"pub enum Msg {
    InputValue(String),
}"#,
        ),
        h3(vec![text("使用 "), code("JsCast")]),
        p(vec![
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html",
                vec![code("wasm-bindgen")],
            ),
            text(" crate 有一個有用的trait："),
            link(
                "https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html",
                vec![code("JsCast")],
            ),
            text("，它允許我們在類型之間直接轉換，只要它實現了"),
            code("JsCast"),
            text(" 就行。我們可以謹慎地轉換，這涉及運行時檢查和處理 "),
            code("Option"),
            text(" 和 "),
            code("Result"),
            text(" 的邏輯，或者我們也可以冒險直接強行轉換。"),
        ]),
        p(vec![text("多說無益，看代碼：")]),
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
        p(vec![
            code("JsCast"),
            text(" 提供的方法是 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into",
                vec![code("dyn_into")],
            ),
            text(" 和 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.unchecked_into",
                vec![code("unchecked_into")],
            ),
            text("。\n如你所見，它們允許我們從 "),
            code("EventTarget"),
            text(" 轉換為 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.HtmlInputElement.html",
                vec![code("HtmlInputElement")],
            ),
            text("。\n"),
            code("dyn_into"),
            text(" 方法是謹慎的，因為它會在運行時檢查類型是否實際為 "),
            code("HtmlInputElement"),
            text("，如果不是則返回\n"),
            code("Err(JsValue)"),
            text("。 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html",
                vec![code("JsValue")],
            ),
            text("\n是一個通用類型，將原來的物件回傳給你，以便再次嘗試轉換為別的類型。"),
        ]),
        p(vec![
            text("這會兒你可能會想，什麼時候可以使用危險版本？在上面的情況下，它是安全的"),
            sup(vec![text("1")]),
            text("，因為我們將 "),
            code("Callback"),
            text(" 設定在一個沒有子元素的元素上，所以目標只能是同一個元素。"),
        ]),
        p(vec![italic(vec![
            sup(vec![text("1")]),
            text(" 只要牽涉到 JS 領域，就是安全的。"),
        ])]),
        h3(vec![text("使用 "), code("TargetCast")]),
        p(vec![bold(vec![
            text("強烈建議先閱讀 "),
            link("#using-jscast", vec![text("使用 JsCast")]),
            text("！"),
        ])]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![
                p(vec![
                    code("TargetCast"),
                    text(" 的設計目的是讓新用戶了解 "),
                    code("JsCast"),
                    text(" 的行為，但範圍更小，僅涉及事件及其目標。"),
                ]),
                p(vec![
                    text("選用 "),
                    code("TargetCast"),
                    text(" 或 "),
                    code("JsCast"),
                    text(" 純粹是個人偏好，實際您會發現 "),
                    code("TargetCast"),
                    text(" 的實作和 "),
                    code("JsCast"),
                    text(" 的功能很相似。"),
                ]),
            ],
        ),
        p(vec![
            code("TargetCast"),
            text(" trait 是在 "),
            code("JsCast"),
            text(" 基礎之上建構的，專門用於從事件中取得類型化的事件目標。"),
        ]),
        p(vec![
            code("TargetCast"),
            text(" 是 Yew 的一部分，因此無需添加依賴項即可在事件上使用 trait 方法，但它的工作方式與 "),
            code("JsCast"),
            text(" 非常相似。"),
        ]),
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
        p(vec![
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
        ]),
        h3(vec![text("使用 "), code("NodeRef")]),
        p(vec![
            link("/zh-Hant/docs/concepts/function-components/node-refs", vec![code("NodeRef")]),
            text(" 可以取代查詢給定給 "),
            code("Callback"),
            text(" 的事件。"),
        ]),
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
        p(vec![
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
        ]),
        p(vec![
            text("你可能會看到，透過使用 "),
            code("NodeRef"),
            text("，我們不必將 "),
            code("String"),
            text(" 傳回狀態，因為我們總是存取 "),
            code("input_node_ref"),
            text(" - 因此我們可以這樣做："),
        ]),
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
        p(vec![
            text("您選擇哪種方法取決於您的元件和您的偏好，沒有所謂的"),
            italic(vec![text("推薦")]),
            text("方法。"),
        ]),
        h2(vec![text("手動事件監聽器")]),
        p(vec![
            text("您可能想要監聽 Yew 的 "),
            code("html"),
            text(" 巨集不支援的事件，請查看"),
            link(
                "#event-types",
                vec![text("這裡列出的支援的事件")],
            ),
            text("。"),
        ]),
        p(vec![
            text("為了手動為某個元素新增事件監聽器，我們需要藉助 "),
            link("/zh-Hant/docs/concepts/function-components/node-refs", vec![code("NodeRef")]),
            text("，以便在 "),
            code("use_effect_with"),
            text(" 中使用 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/index.html",
                vec![code("web-sys")],
            ),
            text(" 和 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html",
                vec![text("wasm-bindgen")],
            ),
            text(" API 新增監聽器。"),
        ]),
        p(vec![
            text("以下範例將展示如何為虛構的 "),
            code("custard"),
            text(" 事件新增監聽器。所有不受 yew 支援的事件或自訂事件都可以表示為\n"),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html",
                vec![code("web_sys::Event")],
            ),
            text("。如果您需要存取自訂/不受支援事件的特定方法或字段，可以使用\n"),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html",
                vec![code("JsCast")],
            ),
            text(" 的方法將其轉換為所需的類型。"),
        ]),
        h3(vec![text("使用 "), code("Closure"), text("（冗長版本）")]),
        p(vec![
            text("直接使用 "),
            code("web-sys"),
            text(" 和 "),
            code("wasm-bindgen"),
            text(" 的介面可能有點痛苦…所以要有點心理準備（"),
            link(
                "#using-gloo-concise",
                vec![text("感謝 gloo，有了更簡潔的方法")],
            ),
            text("）。"),
        ]),
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
        p(vec![
            text("有關 "),
            code("Closure"),
            text(" 的更多信息，請參見 "),
            link(
                "https://wasm-bindgen.github.io/wasm-bindgen/examples/closures.html",
                vec![text("wasm-bindgen 指南")],
            ),
            text("。"),
        ]),
        h3(vec![text("使用 "), code("gloo"), text("（簡潔版本）")]),
        p(vec![
            text("更方便的方法是使用 "),
            code("gloo"),
            text("，更具體地說是 "),
            link(
                "https://docs.rs/gloo-events/0.1.1/gloo_events/index.html",
                vec![code("gloo_events")],
            ),
            text("， 它是 "),
            code("web-sys"),
            text("、"),
            code("wasm-bindgen"),
            text(" 的高層抽象實作。"),
        ]),
        p(vec![
            code("gloo_events"),
            text(" 提供了 "),
            code("EventListener"),
            text(" 類型，可以用來建立和儲存事件監聽器。"),
        ]),
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
        p(vec![
            text("有關 "),
            code("EventListener"),
            text(" 的更多信息，請參見 "),
            link(
                "https://docs.rs/gloo-events/0.1.1/gloo_events/struct.EventListener.html",
                vec![text("gloo_events docs.rs")],
            ),
            text("。"),
        ]),
        h2(vec![text("可用事件的完整清單")]),
        table(
            vec![vec![text("偵聽器名稱")], vec![code("web_sys"), text(" 事件類型")]],
            vec![
                vec![vec![code("onabort")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onauxclick")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", vec![text("MouseEvent")])]],
                vec![vec![code("onblur")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", vec![text("FocusEvent")])]],
                vec![vec![code("oncancel")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("oncanplay")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("oncanplaythrough")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onchange")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onclick")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", vec![text("MouseEvent")])]],
                vec![vec![code("onclose")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("oncontextmenu")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", vec![text("MouseEvent")])]],
                vec![vec![code("oncuechange")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("ondblclick")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", vec![text("MouseEvent")])]],
                vec![vec![code("ondrag")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", vec![text("DragEvent")])]],
                vec![vec![code("ondragend")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", vec![text("DragEvent")])]],
                vec![vec![code("ondragenter")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", vec![text("DragEvent")])]],
                vec![vec![code("ondragexit")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", vec![text("DragEvent")])]],
                vec![vec![code("ondragleave")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", vec![text("DragEvent")])]],
                vec![vec![code("ondragover")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", vec![text("DragEvent")])]],
                vec![vec![code("ondragstart")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", vec![text("DragEvent")])]],
                vec![vec![code("ondrop")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", vec![text("DragEvent")])]],
                vec![vec![code("ondurationchange")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onemptied")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onended")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onerror")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onfocus")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", vec![text("FocusEvent")])]],
                vec![vec![code("onfocusin")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", vec![text("FocusEvent")])]],
                vec![vec![code("onfocusout")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", vec![text("FocusEvent")])]],
                vec![vec![code("onformdata")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("oninput")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.InputEvent.html", vec![text("InputEvent")])]],
                vec![vec![code("oninvalid")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onkeydown")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html", vec![text("KeyboardEvent")])]],
                vec![vec![code("onkeypress")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html", vec![text("KeyboardEvent")])]],
                vec![vec![code("onkeyup")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html", vec![text("KeyboardEvent")])]],
                vec![vec![code("onload")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onloadeddata")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onloadedmetadata")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onloadstart")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html", vec![text("ProgressEvent")])]],
                vec![vec![code("onmousedown")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", vec![text("MouseEvent")])]],
                vec![vec![code("onmouseenter")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", vec![text("MouseEvent")])]],
                vec![vec![code("onmouseleave")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", vec![text("MouseEvent")])]],
                vec![vec![code("onmousemove")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", vec![text("MouseEvent")])]],
                vec![vec![code("onmouseout")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", vec![text("MouseEvent")])]],
                vec![vec![code("onmouseover")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", vec![text("MouseEvent")])]],
                vec![vec![code("onmouseup")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", vec![text("MouseEvent")])]],
                vec![vec![code("onpause")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onplay")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onplaying")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onprogress")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html", vec![text("ProgressEvent")])]],
                vec![vec![code("onratechange")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onreset")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onresize")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onscroll")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onsecuritypolicyviolation")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onseeked")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onseeking")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onselect")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onslotchange")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onstalled")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onsubmit")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.SubmitEvent.html", vec![text("SubmitEvent")])]],
                vec![vec![code("onsuspend")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("ontimeupdate")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("ontoggle")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onvolumechange")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onwaiting")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onwheel")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.WheelEvent.html", vec![text("WheelEvent")])]],
                vec![vec![code("oncopy")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("oncut")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onpaste")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onanimationcancel")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", vec![text("AnimationEvent")])]],
                vec![vec![code("onanimationend")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", vec![text("AnimationEvent")])]],
                vec![vec![code("onanimationiteration")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", vec![text("AnimationEvent")])]],
                vec![vec![code("onanimationstart")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", vec![text("AnimationEvent")])]],
                vec![vec![code("ongotpointercapture")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", vec![text("PointerEvent")])]],
                vec![vec![code("onloadend")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html", vec![text("ProgressEvent")])]],
                vec![vec![code("onlostpointercapture")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", vec![text("PointerEvent")])]],
                vec![vec![code("onpointercancel")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", vec![text("PointerEvent")])]],
                vec![vec![code("onpointerdown")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", vec![text("PointerEvent")])]],
                vec![vec![code("onpointerenter")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", vec![text("PointerEvent")])]],
                vec![vec![code("onpointerleave")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", vec![text("PointerEvent")])]],
                vec![vec![code("onpointerlockchange")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onpointerlockerror")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onpointermove")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", vec![text("PointerEvent")])]],
                vec![vec![code("onpointerout")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", vec![text("PointerEvent")])]],
                vec![vec![code("onpointerover")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", vec![text("PointerEvent")])]],
                vec![vec![code("onpointerup")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", vec![text("PointerEvent")])]],
                vec![vec![code("onselectionchange")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onselectstart")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("onshow")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", vec![text("Event")])]],
                vec![vec![code("ontouchcancel")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", vec![text("TouchEvent")])]],
                vec![vec![code("ontouchend")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", vec![text("TouchEvent")])]],
                vec![vec![code("ontouchmove")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", vec![text("TouchEvent")])]],
                vec![vec![code("ontouchstart")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", vec![text("TouchEvent")])]],
                vec![vec![code("ontransitioncancel")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", vec![text("TransitionEvent")])]],
                vec![vec![code("ontransitionend")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", vec![text("TransitionEvent")])]],
                vec![vec![code("ontransitionrun")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", vec![text("TransitionEvent")])]],
                vec![vec![code("ontransitionstart")], vec![link("https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", vec![text("TransitionEvent")])]],
            ],
        ),
    ])
}

crate::doc_page!("事件", "/zh-Hant/docs/concepts/html/events", page_content());
