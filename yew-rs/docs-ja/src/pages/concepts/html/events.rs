pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["紹介"],
        p![
            "Yew は ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/",
                code("web-sys"),
            ],
            " クレートと統合されており、このクレートのイベントを使用します。以下の",
            link!["#event-types", "表"],
            "には、",
            code("html!"),
            " マクロで受け入れられるすべての ",
            code("web-sys"),
            " イベントが一覧表示されています。",
        ],
        p![
            "下記の表に記載されていないイベントについても、",
            doc_link![crate::pages::concepts::function_components::callbacks,
                code("Callback"),
            ],
            " を追加してリッスンすることができます。詳細は",
            link!["#manual-event-listener", "手動イベントリスナー"],
            "を参照してください。",
        ],
        h2!["イベントタイプ"],
        admonition![AdmonitionType::Tip, None,
            p![
                "すべてのイベントタイプは ",
                code("yew::events"),
                " に再エクスポートされています。",
                code("yew::events"),
                " のタイプを使用することで、",
                code("web-sys"),
                " を手動でクレートに依存関係として追加するよりも、バージョン互換性を確保しやすくなります。Yew が指定するバージョンと競合するバージョンを使用することがなくなります。",
            ],
        ],
        p![
            "イベントリスナーの名前は、",
            code("html"),
            " マクロでイベント ",
            code("Callback"),
            " を追加する際に期待される名前です：",
        ],
        code_block("rust", r#"use yew::prelude::*;

html! {
    <button onclick={Callback::from(|_| ())}>
    //      ^^^^^^^ event listener name
        { "Click me!" }
    </button>
};"#),
        p![
            "イベント名はリスナー名から \"on\" プレフィックスを削除したもので、したがって ",
            code("onclick"),
            " イベントリスナーは ",
            code("click"),
            " イベントをリッスンします。ページの最後にある",
            link!["#event-types", "完全なイベントリスト"],
            "とそのタイプを参照してください。",
        ],
        h2_id!["event-bubbling", "イベントキャプチャ"],
        p![
            "Yew がディスパッチするイベントは仮想 DOM 階層に従い、リスナーに向かってバブルアップします。現在、リスナーのバブルフェーズのみがサポートされています。仮想 DOM 階層は通常（ただし常にではありません）実際の DOM 階層と同じです。",
            doc_link![crate::pages::advanced_topics::portals, "ポータル"],
            "やその他の高度な技術を扱う際には、この違いが重要です。よく設計されたコンポーネントでは、直感的にイベントは子コンポーネントから親コンポーネントにバブルアップするはずです。これにより、",
            code("html!"),
            " で記述した階層がイベントハンドラによって観察される階層となります。",
        ],
        p!["イベントのバブルアップを避けたい場合は、アプリケーションを起動する前に以下のコードを呼び出すことができます"],
        code_block("rust", r#"yew::set_event_bubbling(false);"#),
        p![
            "アプリケーションを起動する",
            italic!["前に"],
            "。これによりイベント処理が高速化されますが、期待されるイベントを受信しないために一部のコンポーネントが動作しなくなる可能性があります。慎重に使用してください！",
        ],
        h2!["イベントデリゲート"],
        p![
            "驚くかもしれませんが、イベントリスナーはレンダリングされた要素に直接登録されるわけではありません。代わりに、イベントは Yew アプリケーションのサブツリーのルートノードから委譲されます。ただし、イベントはそのネイティブ形式で渡され、合成形式は作成されません。これにより、HTML リスナーで予期されるイベントと Yew で発生するイベントとの間に不一致が生じる可能性があります。",
        ],
        ul![
            li![
                link!["https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.current_target",
                    code("Event::current_target"),
                ],
                " はリスナーが追加された要素ではなく、Yew サブツリーのルートノードを指します。基になる ",
                code("HtmlElement"),
                " にアクセスしたい場合は、",
                doc_link![crate::pages::concepts::function_components::node_refs,
                    code("NodeRef"),
                ],
                " を使用してください。",
            ],
            li![
                link!["https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.event_phase",
                    code("Event::event_phase"),
                ],
                " は常に ",
                code("Event::CAPTURING_PHASE"),
                " です。内部的には、イベントはバブリングフェーズにあるかのように振る舞い、イベント伝播が再生され、イベントは",
                link!["#event-bubbling", "上位にバブルアップ"],
                "します。つまり、仮想 DOM 内の上位のイベントリスナーが下位のイベントリスナーの後にトリガーされます。現在、Yew はキャプチャリスナーをサポートしていません。",
            ],
        ],
        p!["これも意味するところは、Yew によって登録されたイベントは通常、他のイベントリスナーよりも先にトリガーされるということです。"],
        h2!["型付きイベントターゲット"],
        admonition![AdmonitionType::Caution, None,
            p![
                "このセクションでは、",
                bold!["target"],
                " (",
                link!["https://developer.mozilla.org/en-US/docs/Web/API/Event/target",
                    code("Event.target"),
                ],
                ") は常にイベントが発生した要素を指します。",
            ],
            p![
                "これは",
                bold!["必ずしも"],
                " ",
                code("Callback"),
                " が配置された要素を指すわけではありません。",
            ],
        ],
        p![
            "イベント ",
            code("Callback"),
            " の中で、イベントのターゲットを取得したい場合があります。例えば、",
            code("change"),
            " イベントは何かが変更されたことを通知するだけで、具体的な情報を提供しません。",
        ],
        p![
            "Yew では、正しい型でターゲット要素を取得する方法がいくつかあり、ここで順を追って説明します。イベント上の ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.target",
                code("web_sys::Event::target"),
            ],
            " を呼び出すと、オプションの ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.EventTarget.html",
                code("web_sys::EventTarget"),
            ],
            " 型が返されますが、入力要素の値を知りたい場合にはあまり役に立たないかもしれません。",
        ],
        p!["以下のすべての方法で、同じ問題を解決します。これにより、方法の違いが明確になり、問題に対処することができます。"],
        p![bold!["問題："]],
        p![
            code("<input>"),
            " 要素に ",
            code("onchange"),
            " ",
            code("Callback"),
            " があり、呼び出されるたびにコンポーネントに更新 ",
            code("Msg"),
            " を送信したいとします。",
        ],
        p![
            code("Msg"),
            " 列挙型は次のようになります：",
        ],
        code_block("rust", r#"pub enum Msg {
    InputValue(String),
}"#),
        h3!["JsCast の使用"],
        p![
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html",
                code("wasm-bindgen"),
            ],
            " クレートには便利なトレイトがあります：",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html",
                code("JsCast"),
            ],
            "。これにより、型が ",
            code("JsCast"),
            " を実装している限り、型間の直接キャストが可能になります。慎重にキャストすることもできますが、これはランタイムチェックと ",
            code("Option"),
            " や ",
            code("Result"),
            " のロジックを処理することを伴います。また、強制的にキャストすることもできます。",
        ],
        p!["コードを見てみましょう："],
        code_block_title("toml", "Cargo.toml", r#"[dependencies]
# JsCast を呼び出すために wasm-bindgen が必要です
wasm-bindgen = "0.2""#),
        code_block("rust", r#"//highlight-next-line
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
            // イベントが作成されたとき、ターゲットは未定義であり、ディスパッチされるときにのみターゲットが追加されます。
            let target: Option<EventTarget> = e.target();
            // イベントはバブルアップする可能性があるため、
            // このリスナーは HtmlInputElement 型ではない子要素のイベントをキャッチする可能性があります。
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
        // target が HtmlInputElement であることを理解している必要があります。
        // そうでない場合、value を呼び出すと未定義の動作（UB）になります。
        // ここでは、これが入力要素であることを確信しているため、チェックせずに適切な型に変換できます。
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
}"#),
        p![
            code("JsCast"),
            " が提供するメソッドは ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into",
                code("dyn_into"),
            ],
            " と ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.unchecked_into",
                code("unchecked_into"),
            ],
            " です。これらのメソッドを使用すると、",
            code("EventTarget"),
            " から ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.HtmlInputElement.html",
                code("HtmlInputElement"),
            ],
            " への変換が可能になります。",
            code("dyn_into"),
            " メソッドは慎重で、実行時に型が実際に ",
            code("HtmlInputElement"),
            " であるかどうかをチェックし、そうでない場合は ",
            code("Err(JsValue)"),
            " を返します。",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html",
                code("JsValue"),
            ],
            " は汎用型で、元のオブジェクトを返し、別の型への変換を再試行することができます。",
        ],
        p![
            "ここで、危険なバージョンを使用するタイミングについて考えるかもしれません。上記のケースでは、子要素のない要素に ",
            code("Callback"),
            " を設定しているため、ターゲットは同じ要素である必要があるため、安全です",
            sup!["1"],
            "。",
        ],
        p![
            italic![sup!["1"], " JS の領域に関わる限り、安全です。"],
        ],
        h3!["TargetCast の使用"],
        p![
            bold!["JsCast の使用を先に読むことを強くお勧めします！"],
        ],
        admonition![AdmonitionType::Note, None,
            p![
                code("TargetCast"),
                " は新しいユーザーが ",
                code("JsCast"),
                " の動作を理解するために設計されていますが、範囲はイベントとそのターゲットに限定されています。",
            ],
            p![
                code("TargetCast"),
                " または ",
                code("JsCast"),
                " を選択するのは純粋に個人の好みの問題であり、実際には ",
                code("TargetCast"),
                " の実装と ",
                code("JsCast"),
                " の機能は非常に似ています。",
            ],
        ],
        p![
            code("TargetCast"),
            " トレイトは ",
            code("JsCast"),
            " の上に構築されており、イベントから型付きのイベントターゲットを取得するために特化されています。",
        ],
        p![
            code("TargetCast"),
            " は Yew の一部であるため、依存関係を追加せずにイベント上でトレイトメソッドを使用できますが、その動作は ",
            code("JsCast"),
            " と非常に似ています。",
        ],
        code_block("rust", r#"use web_sys::HtmlInputElement;
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
        // target が HtmlInputElement であることを理解している必要があります。
        // そうでない場合、value を呼び出すと未定義の動作（UB）になります。
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
}"#),
        p![
            "もし ",
            code("JsCast"),
            " についてすでに知っているか、このトレイトに精通している場合、",
            code("TargetCast::target_dyn_into"),
            " が ",
            code("JsCast::dyn_into"),
            " に似ていることに気付くでしょうが、イベントのターゲットに特化しています。",
            code("TargetCast::target_unchecked_into"),
            " は ",
            code("JsCast::unchecked_into"),
            " に似ているため、上記の ",
            code("JsCast"),
            " に関するすべての警告が ",
            code("TargetCast"),
            " にも適用されます。",
        ],
        h3!["NodeRef の使用"],
        p![
            doc_link![crate::pages::concepts::function_components::node_refs,
                code("NodeRef"),
            ],
            " は、与えられたイベントを ",
            code("Callback"),
            " に渡す代わりに使用できます。",
        ],
        code_block("rust", r#"use web_sys::HtmlInputElement;
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
}"#),
        p![
            code("NodeRef"),
            " を使用すると、イベントを無視して ",
            code("NodeRef::cast"),
            " メソッドを使用して ",
            code("Option<HtmlInputElement>"),
            " を取得できます。これはオプションであり、",
            code("NodeRef"),
            " を設定する前に ",
            code("cast"),
            " を呼び出すか、型が一致しない場合に ",
            code("None"),
            " を返します。",
        ],
        p![
            code("NodeRef"),
            " を使用することで、常に ",
            code("input_node_ref"),
            " にアクセスできるため、状態に ",
            code("String"),
            " を送信する必要がないことがわかるかもしれません。したがって、次のようにすることができます：",
        ],
        code_block("rust", r#"use web_sys::HtmlInputElement;
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
}"#),
        p!["どの方法を選択するかは、コンポーネントと個人の好みによります。推奨される方法はありません。"],
        h2_id!["manual-event-listener", "手動イベントリスナー"],
        p![
            "Yew の ",
            code("html"),
            " マクロがサポートしていないイベントをリッスンしたい場合があります。サポートされているイベントのリストは",
            link!["#event-types", "こちら"],
            "を参照してください。",
        ],
        p![
            "手動で要素にイベントリスナーを追加するには、",
            doc_link![crate::pages::concepts::function_components::node_refs,
                code("NodeRef"),
            ],
            " を使用して、",
            code("use_effect_with"),
            " 内で ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/index.html",
                code("web-sys"),
            ],
            " と ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html",
                "wasm-bindgen",
            ],
            " API を使用してリスナーを追加します。",
        ],
        p![
            "以下の例では、架空の ",
            code("custard"),
            " イベントにリスナーを追加する方法を示します。Yew がサポートしていないすべてのイベントやカスタムイベントは、",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html",
                code("web_sys::Event"),
            ],
            " として表現できます。カスタム/サポートされていないイベントの特定のメソッドやフィールドにアクセスする必要がある場合は、",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html",
                code("JsCast"),
            ],
            " のメソッドを使用して必要な型に変換できます。",
        ],
        h3!["Closure を使用する（冗長バージョン）"],
        p![
            "直接 ",
            code("web-sys"),
            " と ",
            code("wasm-bindgen"),
            " のインターフェースを使用するのは少し面倒かもしれません……なので、心の準備をしてください（",
            link!["#using-gloo-concise", code("gloo"), " のおかげで、より簡潔な方法があります"],
            "）。",
        ],
        code_block("rust", r#"use wasm_bindgen::{prelude::Closure, JsCast};
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
                    // 通常作成する Callback を作成
                    let oncustard = Callback::from(move |_: Event| {
                        // カスタードに対して何かを行う..
                    });

                    // Box<dyn Fn> から Closure を作成 - これは 'static である必要があります
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
}"#),
        p![
            code("Closure"),
            " の詳細については、",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/examples/closures.html", "wasm-bindgen ガイド"],
            " を参照してください。",
        ],
        h3_id!["using-gloo-concise", "gloo を使用する（簡潔なバージョン）"],
        p![
            "より便利な方法は、",
            code("gloo"),
            "、具体的には ",
            link!["https://docs.rs/gloo-events/0.1.1/gloo_events/index.html",
                code("gloo_events"),
            ],
            " を使用することです。 これは ",
            code("web-sys"),
            "、",
            code("wasm-bindgen"),
            " の高レベル抽象実装です。",
        ],
        p![
            code("gloo_events"),
            " は、イベントリスナーを作成および保存するために使用できる ",
            code("EventListener"),
            " 型を提供します。",
        ],
        code_block_title("toml", "Cargo.toml", r#"[dependencies]
gloo-events = "0.1""#),
        code_block("rust", r#"use web_sys::HtmlElement;
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
                    // 通常作成する Callback を作成
                    let oncustard = Callback::from(move |_: Event| {
                        // カスタードに対して何かを行う..
                    });

                    // Box<dyn Fn> から Closure を作成 - これは 'static である必要があります
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
}"#),
        p![
            code("EventListener"),
            " の詳細については、",
            link!["https://docs.rs/gloo-events/0.1.1/gloo_events/struct.EventListener.html", "gloo_events docs.rs"],
            " を参照してください。",
        ],
        h2_id!["available-events", "利用可能なイベントの完全なリスト"],
        table(
            vec![vec!["リスナー名".into()], vec![code("web_sys"), " イベントの種類".into()]],
            vec![
                vec![vec![code("onabort")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onauxclick")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("onblur")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", "FocusEvent"]]],
                vec![vec![code("oncancel")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("oncanplay")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("oncanplaythrough")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onchange")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onclick")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("onclose")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("oncontextmenu")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("oncuechange")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("ondblclick")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("ondrag")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"]]],
                vec![vec![code("ondragend")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"]]],
                vec![vec![code("ondragenter")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"]]],
                vec![vec![code("ondragexit")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"]]],
                vec![vec![code("ondragleave")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"]]],
                vec![vec![code("ondragover")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"]]],
                vec![vec![code("ondragstart")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"]]],
                vec![vec![code("ondrop")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"]]],
                vec![vec![code("ondurationchange")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onemptied")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onended")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onerror")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onfocus")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", "FocusEvent"]]],
                vec![vec![code("onfocusin")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", "FocusEvent"]]],
                vec![vec![code("onfocusout")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", "FocusEvent"]]],
                vec![vec![code("onformdata")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("oninput")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.InputEvent.html", "InputEvent"]]],
                vec![vec![code("oninvalid")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onkeydown")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html", "KeyboardEvent"]]],
                vec![vec![code("onkeypress")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html", "KeyboardEvent"]]],
                vec![vec![code("onkeyup")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html", "KeyboardEvent"]]],
                vec![vec![code("onload")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onloadeddata")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onloadedmetadata")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onloadstart")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html", "ProgressEvent"]]],
                vec![vec![code("onmousedown")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("onmouseenter")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("onmouseleave")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("onmousemove")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("onmouseout")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("onmouseover")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("onmouseup")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("onpause")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onplay")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onplaying")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onprogress")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html", "ProgressEvent"]]],
                vec![vec![code("onratechange")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onreset")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onresize")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onscroll")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onsecuritypolicyviolation")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onseeked")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onseeking")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onselect")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onslotchange")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onstalled")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onsubmit")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.SubmitEvent.html", "SubmitEvent"]]],
                vec![vec![code("onsuspend")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("ontimeupdate")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("ontoggle")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onvolumechange")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onwaiting")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onwheel")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.WheelEvent.html", "WheelEvent"]]],
                vec![vec![code("oncopy")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("oncut")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onpaste")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onanimationcancel")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", "AnimationEvent"]]],
                vec![vec![code("onanimationend")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", "AnimationEvent"]]],
                vec![vec![code("onanimationiteration")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", "AnimationEvent"]]],
                vec![vec![code("onanimationstart")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", "AnimationEvent"]]],
                vec![vec![code("ongotpointercapture")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onloadend")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html", "ProgressEvent"]]],
                vec![vec![code("onlostpointercapture")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onpointercancel")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onpointerdown")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onpointerenter")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onpointerleave")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onpointerlockchange")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onpointerlockerror")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onpointermove")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onpointerout")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onpointerover")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onpointerup")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onselectionchange")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onselectstart")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onshow")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("ontouchcancel")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", "TouchEvent"]]],
                vec![vec![code("ontouchend")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", "TouchEvent"]]],
                vec![vec![code("ontouchmove")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", "TouchEvent"]]],
                vec![vec![code("ontouchstart")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", "TouchEvent"]]],
                vec![vec![code("ontransitioncancel")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", "TransitionEvent"]]],
                vec![vec![code("ontransitionend")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", "TransitionEvent"]]],
                vec![vec![code("ontransitionrun")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", "TransitionEvent"]]],
                vec![vec![code("ontransitionstart")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", "TransitionEvent"]]],
            ],
        ),
    ])
}

crate::doc_page!("イベント", "/ja/docs/concepts/html/events", page_content());
