crate::doc_page!(
    "Elements",
    "/zh-Hant/docs/concepts/html/elements",
    Content::new(vec![
        h2!["標籤結構"],
        p![
            "元件標籤都必須要是自封閉的標籤 ",
            code("<... />"),
            " 或是跟開啟標籤對應的關閉標籤。",
        ],
        tabs!(
            "Open - Close",
            tab!(
                "Open - Close",
                "Open - Close",
                code_block("rust", "html! {\n  <div id=\"my_div\"></div>\n}",),
            ),
            tab!(
                "INVALID",
                "INVALID",
                code_block(
                    "rust",
                    "html! {\n  <div id=\"my_div\"> // <- 缺少關閉標籤\n}",
                ),
            ),
        ),
        tabs!(
            "Self-Closing",
            tab!(
                "Self-Closing",
                "Self-Closing",
                code_block("rust", "html! {\n  <input id=\"my_input\" />\n}",),
            ),
            tab!(
                "INVALID2",
                "INVALID",
                code_block(
                    "rust",
                    "html! {\n  <input id=\"my_input\"> // <- 缺少自封閉標籤語法\\
                     n}",
                ),
            ),
        ),
        admonition!(
            AdmonitionType::Note,
            None,
            p![
                "為了方便起見，通常\\
                     u{9700}要關閉標籤的元件\\
                     u{FF0C}也都可以用自封閉\\
                     u{6A19}籤表示。例如，寫 ",
                code("html! { <div class=\"placeholder\" /> }"),
                " 是合法的。",
            ],
        ),
        h2!["子結點"],
        p!["輕鬆寫出複雜巢狀的 HTML 與 SVG 架構："],
        tabs!(
            "HTML",
            tab!(
                "HTML",
                "HTML",
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
}"#,
                ),
            ),
            tab!(
                "SVG",
                "SVG",
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
}"##,
                ),
            ),
        ),
        h2!["Classes"],
        p!["你很多方便的選項可以\\
             u{5BEB}元件裡的 class："],
        tabs!(
            "Literal",
            tab!(
                "Literal",
                "Literal",
                code_block("rust", "html! {\n  <div class=\"container\"></div>\n}",),
            ),
            tab!(
                "Multiple",
                "Multiple",
                code_block(
                    "rust",
                    "html! {\n  <div class=\"container center-align\"></div>\n}",
                ),
            ),
            tab!(
                "Interpolated",
                "Interpolated",
                code_block(
                    "rust",
                    "html! {\n  <div class={format!(\"{}-container\", size)}></div>\n}",
                ),
            ),
            tab!(
                "Expression",
                "Expression",
                code_block("rust", "html! {\n  <div class={self.classes()}></div>\n}",),
            ),
            tab!(
                "Tuple",
                "Tuple",
                code_block(
                    "rust",
                    "html! {\n  <div class={(\"class-1\", \"class-2\")}></div>\n}",
                ),
            ),
            tab!(
                "Vector",
                "Vector",
                code_block(
                    "rust",
                    "html! {\n  <div class={vec![\"class-1\", \"class-2\"]}></div>\n}",
                ),
            ),
        ),
        h2!["監聽"],
        p![
            "監聽器的屬性必須要傳\\
                 u{5165}一個 ",
            code("Callback"),
            " ，他封裝了閉包。callback 的內容取決於，當觸發\\
                 u{76E3}聽事件時，你希望應\\
                 u{7528}程式有什麼反應：",
        ],
        tabs!(
            "Component Handler",
            tab!(
                "Component Handler",
                "Component Handler",
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
                ),
            ),
            tab!(
                "Agent Handler",
                "Agent Handler",
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
        let click_callback = self.worker.callback(|_: ClickEvent| WorkerMsg::Process);
        html! {
            <button onclick={click_callback}>
                { "Click me!" }
            </button>
        }
    }
}"#,
                ),
            ),
            tab!(
                "Other Cases",
                "Other Cases",
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
                ),
            ),
        ),
    ])
);
