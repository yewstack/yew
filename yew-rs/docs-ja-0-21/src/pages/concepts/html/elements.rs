crate::doc_page!(
    "Elements",
    "/ja/docs/concepts/html/elements",
    Content::new(vec![
        h2(vec![text("タグ構造")]),
        p(vec![text(
            "要素のタグは自己完結タグか、開始タグに対応した終了タグを持っている必要があります。"
        )]),
        p(vec![text("Open - Close:")]),
        code_block(
            "rust",
            r#"html! {
  <div id="my_div"></div>
}"#
        ),
        p(vec![text("Invalid:")]),
        code_block(
            "rust",
            r#"html! {
  <div id="my_div"> // <- MISSING CLOSE TAG
}"#
        ),
        p(vec![text("Self-closing:")]),
        code_block(
            "rust",
            r#"html! {
  <input id="my_input" />
}"#
        ),
        p(vec![text("Invalid:")]),
        code_block(
            "rust",
            r#"html! {
  <input id="my_input"> // <- MISSING SELF-CLOSE
}"#
        ),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text("便利さのために、"),
                italic(vec![text("普通は")]),
                text("終了タグを必要とする要素は自己完結タグとすることが"),
                bold(vec![text("できます")]),
                text("。例えば"),
                code("html! { <div class=\"placeholder\" /> }"),
                text("と書くのは有効です。"),
            ])]
        ),
        h2(vec![text("子")]),
        p(vec![text(
            "複雑にネストした HTML や SVG のレイアウトを書くのには以下のようにするのが楽です:"
        )]),
        p(vec![text("HTML:")]),
        code_block(
            "rust",
            r#"html! {
    <div>
        <div data-key="abc"></div>
        <div class="parent">
            <span class="child" value="anything"></span>
            <label for="first-name">{ "First Name" }</label>
            <input type="text" id="first-name" value="placeholder" />
            <input type="checkbox" checked=true />
            <textarea value="write a story" />
            <select name="status">
                <option selected=true disabled=false value="">{ "Selected" }</option>
                <option selected=false disabled=true value="">{ "Unselected" }</option>
            </select>
        </div>
    </div>
}"#
        ),
        p(vec![text("SVG:")]),
        code_block(
            "rust",
            r##"html! {
    <svg width="149" height="147" viewBox="0 0 149 147" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M60.5776 13.8268L51.8673 42.6431L77.7475 37.331L60.5776 13.8268Z" fill="#DEB819"/>
        <path d="M108.361 94.9937L138.708 90.686L115.342 69.8642" stroke="black" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
        <g filter="url(#filter0_d)">
            <circle cx="75.3326" cy="73.4918" r="55" fill="#FDD630"/>
            <circle cx="75.3326" cy="73.4918" r="52.5" stroke="black" stroke-width="5"/>
        </g>
        <circle cx="71" cy="99" r="5" fill="white" fill-opacity="0.75" stroke="black" stroke-width="3"/>
        <defs>
            <filter id="filter0_d" x="16.3326" y="18.4918" width="118" height="118" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
                <feGaussianBlur stdDeviation="2"/>
                <feColorMatrix in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"/>
            </filter>
        </defs>
    </svg>
}"##
        ),
        h2(vec![text("クラス")]),
        p(vec![text(
            "要素へのクラスを特定する便利なやり方はたくさんあります:"
        )]),
        p(vec![text("Literal:")]),
        code_block(
            "rust",
            r#"html! {
  <div class="container"></div>
}"#
        ),
        p(vec![text("Multiple:")]),
        code_block(
            "rust",
            r#"html! {
  <div class="container center-align"></div>
}"#
        ),
        p(vec![text("Interpolated:")]),
        code_block(
            "rust",
            r#"html! {
  <div class={format!("{}-container", size)}></div>
}"#
        ),
        p(vec![text("Expression:")]),
        code_block(
            "rust",
            r#"html! {
  <div class={self.classes()}></div>
}"#
        ),
        p(vec![text("Tuple:")]),
        code_block(
            "rust",
            r#"html! {
  <div class={("class-1", "class-2")}></div>
}"#
        ),
        p(vec![text("Vector:")]),
        code_block(
            "rust",
            r#"html! {
  <div class={vec!["class-1", "class-2"]}></div>
}"#
        ),
        h2(vec![text("リスナー")]),
        p(vec![
            text("リスナー属性はクロージャのラッパーである"),
            code("Callback"),
            text("に渡される必要があります。コールバックをどのように作るかは\
                  アプリをリスナーイベントにどう反応させたいかによります。"),
        ]),
        p(vec![text("Component handler:")]),
        code_block(
            "rust",
            r#"struct MyComponent {
    link: ComponentLink<Self>,
}

enum Msg {
    Click,
}

impl Component for MyComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        MyComponent { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                // Handle Click
            }
        }
    }

    fn view(&self) -> Html {
        // Create a callback from a component link to handle it in a component
        let click_callback = self.link.callback(|_: ClickEvent| Msg::Click);
        html! {
            <button onclick={click_callback}>
                { "Click me!" }
            </button>
        }
    }
}"#
        ),
        p(vec![text("Agent Handler:")]),
        code_block(
            "rust",
            r#"struct MyComponent {
    worker: Dispatcher<MyWorker>,
}

impl Component for MyComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        MyComponent {
            worker: MyWorker::dispatcher()
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // Create a callback from a worker to handle it in another context
        let click_callback = self.worker.callback(|_: ClickEvent| WorkerMsg::Process);
        html! {
            <button onclick={click_callback}>
                { "Click me!" }
            </button>
        }
    }
}"#
        ),
        p(vec![text("Other Cases:")]),
        code_block(
            "rust",
            r#"struct MyComponent;

impl Component for MyComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        MyComponent
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // Create an ephemeral callback
        let click_callback = Callback::from(|| {
            ConsoleService::log("clicked!");
        });

        html! {
            <button onclick={click_callback}>
                { "Click me!" }
            </button>
        }
    }
}"#
        ),
        h2(vec![text("イベントの型")]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text("以下のテーブルにある全てのイベントの型は"),
                code("yew::events"),
                text("で再エクスポートされています。"),
            ])]
        ),
        table(
            vec![
                vec![text("イベント名")],
                vec![code("web_sys"), text(" イベント型")],
            ],
            vec![
                vec![
                    vec![code("onabort")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onauxclick")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html",
                        vec![text("MouseEvent")]
                    )]
                ],
                vec![
                    vec![code("onblur")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html",
                        vec![text("FocusEvent")]
                    )]
                ],
                vec![
                    vec![code("oncancel")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("oncanplay")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("oncanplaythrough")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onchange")],
                    vec![link(
                        "https://docs.rs/yew/latest/yew/events/enum.ChangeData.html",
                        vec![text("ChangeData")]
                    )]
                ],
                vec![
                    vec![code("onclick")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html",
                        vec![text("MouseEvent")]
                    )]
                ],
                vec![
                    vec![code("onclose")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("oncontextmenu")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html",
                        vec![text("MouseEvent")]
                    )]
                ],
                vec![
                    vec![code("oncuechange")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("ondblclick")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html",
                        vec![text("MouseEvent")]
                    )]
                ],
                vec![
                    vec![code("ondrag")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html",
                        vec![text("DragEvent")]
                    )]
                ],
                vec![
                    vec![code("ondragend")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html",
                        vec![text("DragEvent")]
                    )]
                ],
                vec![
                    vec![code("ondragenter")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html",
                        vec![text("DragEvent")]
                    )]
                ],
                vec![
                    vec![code("ondragexit")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html",
                        vec![text("DragEvent")]
                    )]
                ],
                vec![
                    vec![code("ondragleave")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html",
                        vec![text("DragEvent")]
                    )]
                ],
                vec![
                    vec![code("ondragover")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html",
                        vec![text("DragEvent")]
                    )]
                ],
                vec![
                    vec![code("ondragstart")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html",
                        vec![text("DragEvent")]
                    )]
                ],
                vec![
                    vec![code("ondrop")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html",
                        vec![text("DragEvent")]
                    )]
                ],
                vec![
                    vec![code("ondurationchange")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onemptied")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onended")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onerror")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onfocus")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html",
                        vec![text("FocusEvent")]
                    )]
                ],
                vec![
                    vec![code("onformdata")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("oninput")],
                    vec![link(
                        "https://docs.rs/yew/latest/yew/events/struct.InputData.html",
                        vec![text("InputData")]
                    )]
                ],
                vec![
                    vec![code("oninvalid")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onkeydown")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html",
                        vec![text("KeyboardEvent")]
                    )]
                ],
                vec![
                    vec![code("onkeypress")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html",
                        vec![text("KeyboardEvent")]
                    )]
                ],
                vec![
                    vec![code("onkeyup")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html",
                        vec![text("KeyboardEvent")]
                    )]
                ],
                vec![
                    vec![code("onload")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onloadeddata")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onloadedmetadata")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onloadstart")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html",
                        vec![text("ProgressEvent")]
                    )]
                ],
                vec![
                    vec![code("onmousedown")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html",
                        vec![text("MouseEvent")]
                    )]
                ],
                vec![
                    vec![code("onmouseenter")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html",
                        vec![text("MouseEvent")]
                    )]
                ],
                vec![
                    vec![code("onmouseleave")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html",
                        vec![text("MouseEvent")]
                    )]
                ],
                vec![
                    vec![code("onmousemove")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html",
                        vec![text("MouseEvent")]
                    )]
                ],
                vec![
                    vec![code("onmouseout")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html",
                        vec![text("MouseEvent")]
                    )]
                ],
                vec![
                    vec![code("onmouseover")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html",
                        vec![text("MouseEvent")]
                    )]
                ],
                vec![
                    vec![code("onmouseup")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html",
                        vec![text("MouseEvent")]
                    )]
                ],
                vec![
                    vec![code("onpause")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onplay")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onplaying")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onprogress")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html",
                        vec![text("ProgressEvent")]
                    )]
                ],
                vec![
                    vec![code("onratechange")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onreset")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onresize")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onscroll")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onsecuritypolicyviolation")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onseeked")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onseeking")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onselect")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onslotchange")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onstalled")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onsubmit")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html",
                        vec![text("FocusEvent")]
                    )]
                ],
                vec![
                    vec![code("onsuspend")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("ontimeupdate")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("ontoggle")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onvolumechange")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onwaiting")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onwheel")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.WheelEvent.html",
                        vec![text("WheelEvent")]
                    )]
                ],
                vec![
                    vec![code("oncopy")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("oncut")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onpaste")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onanimationcancel")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html",
                        vec![text("AnimationEvent")]
                    )]
                ],
                vec![
                    vec![code("onanimationend")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html",
                        vec![text("AnimationEvent")]
                    )]
                ],
                vec![
                    vec![code("onanimationiteration")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html",
                        vec![text("AnimationEvent")]
                    )]
                ],
                vec![
                    vec![code("onanimationstart")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html",
                        vec![text("AnimationEvent")]
                    )]
                ],
                vec![
                    vec![code("ongotpointercapture")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html",
                        vec![text("PointerEvent")]
                    )]
                ],
                vec![
                    vec![code("onloadend")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html",
                        vec![text("ProgressEvent")]
                    )]
                ],
                vec![
                    vec![code("onlostpointercapture")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html",
                        vec![text("PointerEvent")]
                    )]
                ],
                vec![
                    vec![code("onpointercancel")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html",
                        vec![text("PointerEvent")]
                    )]
                ],
                vec![
                    vec![code("onpointerdown")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html",
                        vec![text("PointerEvent")]
                    )]
                ],
                vec![
                    vec![code("onpointerenter")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html",
                        vec![text("PointerEvent")]
                    )]
                ],
                vec![
                    vec![code("onpointerleave")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html",
                        vec![text("PointerEvent")]
                    )]
                ],
                vec![
                    vec![code("onpointerlockchange")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onpointerlockerror")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onpointermove")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html",
                        vec![text("PointerEvent")]
                    )]
                ],
                vec![
                    vec![code("onpointerout")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html",
                        vec![text("PointerEvent")]
                    )]
                ],
                vec![
                    vec![code("onpointerover")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html",
                        vec![text("PointerEvent")]
                    )]
                ],
                vec![
                    vec![code("onpointerup")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html",
                        vec![text("PointerEvent")]
                    )]
                ],
                vec![
                    vec![code("onselectionchange")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onselectstart")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("onshow")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.Event.html",
                        vec![text("Event")]
                    )]
                ],
                vec![
                    vec![code("ontouchcancel")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html",
                        vec![text("TouchEvent")]
                    )]
                ],
                vec![
                    vec![code("ontouchend")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html",
                        vec![text("TouchEvent")]
                    )]
                ],
                vec![
                    vec![code("ontouchmove")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html",
                        vec![text("TouchEvent")]
                    )]
                ],
                vec![
                    vec![code("ontouchstart")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html",
                        vec![text("TouchEvent")]
                    )]
                ],
                vec![
                    vec![code("ontransitioncancel")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html",
                        vec![text("TransitionEvent")]
                    )]
                ],
                vec![
                    vec![code("ontransitionend")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html",
                        vec![text("TransitionEvent")]
                    )]
                ],
                vec![
                    vec![code("ontransitionrun")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html",
                        vec![text("TransitionEvent")]
                    )]
                ],
                vec![
                    vec![code("ontransitionstart")],
                    vec![link(
                        "https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html",
                        vec![text("TransitionEvent")]
                    )]
                ],
            ]
        ),
    ])
);
