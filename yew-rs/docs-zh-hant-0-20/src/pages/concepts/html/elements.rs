crate::doc_page!(
    "Elements",
    "/zh-Hant/docs/concepts/html/elements",
    Content::new(vec![
        h2(vec![text("標籤結構")]),
        p(vec![
            text("元件標籤都必須要是自封閉的標籤 "),
            code("<... />"),
            text(" 或是跟開啟標籤對應的關閉標籤。"),
        ]),
        tabs(
            "Open - Close",
            vec![
                tab(
                    "Open - Close",
                    "Open - Close",
                    vec![code_block(
                        "rust",
                        "html! {\n  <div id=\"my_div\"></div>\n}",
                    )],
                ),
                tab(
                    "INVALID",
                    "INVALID",
                    vec![code_block(
                        "rust",
                        "html! {\n  <div id=\"my_div\"> // <- \
                         \u{7F3A}\u{5C11}\u{95DC}\u{9589}\u{6A19}\u{7C64}\n}",
                    )],
                ),
            ],
        ),
        tabs(
            "Self-Closing",
            vec![
                tab(
                    "Self-Closing",
                    "Self-Closing",
                    vec![code_block(
                        "rust",
                        "html! {\n  <input id=\"my_input\" />\n}",
                    )],
                ),
                tab(
                    "INVALID2",
                    "INVALID",
                    vec![code_block(
                        "rust",
                        "html! {\n  <input id=\"my_input\"> // <- \
                         \u{7F3A}\u{5C11}\u{81EA}\u{5C01}\u{9589}\u{6A19}\u{7C64}\u{8A9E}\u{6CD5}\\
                         \
                         n}",
                    )],
                ),
            ],
        ),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text(
                    "\u{70BA}\u{4E86}\u{65B9}\u{4FBF}\u{8D77}\u{898B}\u{FF0C}\u{901A}\u{5E38}\\
                     u{9700}\u{8981}\u{95DC}\u{9589}\u{6A19}\u{7C64}\u{7684}\u{5143}\u{4EF6}\\
                     u{FF0C}\u{4E5F}\u{90FD}\u{53EF}\u{4EE5}\u{7528}\u{81EA}\u{5C01}\u{9589}\\
                     u{6A19}\u{7C64}\u{8868}\u{793A}\u{3002}\u{4F8B}\u{5982}\u{FF0C}\u{5BEB} "
                ),
                code("html! { <div class=\"placeholder\" /> }"),
                text(" \u{662F}\u{5408}\u{6CD5}\u{7684}\u{3002}"),
            ])],
        ),
        h2(vec![text("\u{5B50}\u{7D50}\u{9EDE}")]),
        p(vec![text(
            "\u{8F15}\u{9B06}\u{5BEB}\u{51FA}\u{8907}\u{96DC}\u{5DE2}\u{72C0}\u{7684} HTML \
             \u{8207} SVG \u{67B6}\u{69CB}\u{FF1A}"
        )]),
        tabs(
            "HTML",
            vec![
                tab(
                    "HTML",
                    "HTML",
                    vec![code_block(
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
}"#,
                    )],
                ),
                tab(
                    "SVG",
                    "SVG",
                    vec![code_block(
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
}"##,
                    )],
                ),
            ],
        ),
        h2(vec![text("Classes")]),
        p(vec![text(
            "\u{4F60}\u{5F88}\u{591A}\u{65B9}\u{4FBF}\u{7684}\u{9078}\u{9805}\u{53EF}\u{4EE5}\\
             u{5BEB}\u{5143}\u{4EF6}\u{88E1}\u{7684} class\u{FF1A}"
        )]),
        tabs(
            "Literal",
            vec![
                tab(
                    "Literal",
                    "Literal",
                    vec![code_block(
                        "rust",
                        "html! {\n  <div class=\"container\"></div>\n}"
                    )]
                ),
                tab(
                    "Multiple",
                    "Multiple",
                    vec![code_block(
                        "rust",
                        "html! {\n  <div class=\"container center-align\"></div>\n}"
                    )]
                ),
                tab(
                    "Interpolated",
                    "Interpolated",
                    vec![code_block(
                        "rust",
                        "html! {\n  <div class={format!(\"{}-container\", size)}></div>\n}"
                    )]
                ),
                tab(
                    "Expression",
                    "Expression",
                    vec![code_block(
                        "rust",
                        "html! {\n  <div class={self.classes()}></div>\n}"
                    )]
                ),
                tab(
                    "Tuple",
                    "Tuple",
                    vec![code_block(
                        "rust",
                        "html! {\n  <div class={(\"class-1\", \"class-2\")}></div>\n}"
                    )]
                ),
                tab(
                    "Vector",
                    "Vector",
                    vec![code_block(
                        "rust",
                        "html! {\n  <div class={vec![\"class-1\", \"class-2\"]}></div>\n}"
                    )]
                ),
            ],
        ),
        h2(vec![text("\u{76E3}\u{807D}")]),
        p(vec![
            text(
                "\u{76E3}\u{807D}\u{5668}\u{7684}\u{5C6C}\u{6027}\u{5FC5}\u{9808}\u{8981}\u{50B3}\\
                 \
                 u{5165}\u{4E00}\u{500B} "
            ),
            code("Callback"),
            text(
                " \u{FF0C}\u{4ED6}\u{5C01}\u{88DD}\u{4E86}\u{9589}\u{5305}\u{3002}callback \
                 \u{7684}\u{5167}\u{5BB9}\u{53D6}\u{6C7A}\u{65BC}\u{FF0C}\u{7576}\u{89F8}\u{767C}\\
                 \
                 u{76E3}\u{807D}\u{4E8B}\u{4EF6}\u{6642}\u{FF0C}\u{4F60}\u{5E0C}\u{671B}\u{61C9}\\
                 u{7528}\u{7A0B}\u{5F0F}\u{6709}\u{4EC0}\u{9EBC}\u{53CD}\u{61C9}\u{FF1A}",
            ),
        ]),
        tabs(
            "Component Handler",
            vec![
                tab(
                    "Component Handler",
                    "Component Handler",
                    vec![code_block(
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
            }
        }
    }

    fn view(&self) -> Html {
        let click_callback = self.link.callback(|_: ClickEvent| Msg::Click);
        html! {
            <button onclick={click_callback}>
                { "Click me!" }
            </button>
        }
    }
}"#,
                    )],
                ),
                tab(
                    "Agent Handler",
                    "Agent Handler",
                    vec![code_block(
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
        let click_callback = self.worker.callback(|_: ClickEvent| WorkerMsg::Process);
        html! {
            <button onclick={click_callback}>
                { "Click me!" }
            </button>
        }
    }
}"#,
                    )],
                ),
                tab(
                    "Other Cases",
                    "Other Cases",
                    vec![code_block(
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
        let click_callback = Callback::from(|| {
            ConsoleService::log("clicked!");
        });

        html! {
            <button onclick={click_callback}>
                { "Click me!" }
            </button>
        }
    }
}"#,
                    )],
                ),
            ],
        ),
    ])
);
