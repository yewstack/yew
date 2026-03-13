crate::doc_page!(
    "",
    "/zh-Hans/docs/concepts/html/elements",
    Content::new(vec![
        h1(vec![text("\u{5143}\u{7d20}")]),
        h2(vec![text("\u{6807}\u{7b7e}\u{7ed3}\u{6784}")]),
        p(vec![
            text(
                "\u{5143}\u{7d20}\u{6807}\u{7b7e}\u{5fc5}\u{987b}\u{662f}\u{81ea}\u{95ed}\u{5408}\\
                 \
                 u{7684} "
            ),
            code("<... />"),
            text(
                "\u{ff0c}\u{6216}\u{662f}\u{6bcf}\u{4e2a}\u{6807}\u{7b7e}\u{90fd}\u{6709}\u{4e00}\\
                 \
                 u{4e2a}\u{5bf9}\u{5e94}\u{7684}\u{95ed}\u{5408}\u{6807}\u{7b7e}\u{3002}"
            ),
        ]),
        code_block(
            "rust",
            r#"use web_sys::{Element, Node};
use yew::prelude::*;
use gloo::utils::document;

#[function_component]
fn MyComponent() -> Html {
    // memoize as this only needs to be executed once
    let node = use_memo(
        |_| {
            // Create a div element from the document
            let div: Element = document().create_element("div").unwrap();
            // Add content, classes etc.
            div.set_inner_html("Hello, World!");
            // Convert Element into a Node
            let node: Node = div.into();
            // Return that Node as a Html value
            Html::VRef(node)
        },
        (),
    );

    // use_memo return Rc so we need to deref and clone
    (*node).clone()
}"#
        ),
        code_block(
            "rust",
            r#"use yew::prelude::*;

let level = 5;
let text = "Hello World!".to_owned();

html! {
    <@{format!("h{}", level)} class="title">{ text }</@>
};"#
        ),
        code_block(
            "rust",
            r#"use yew::prelude::*;

html! {
    <div hidden=true>
        { "This div is hidden." }
    </div>
};"#
        ),
        code_block("rust", r#"<div hidden>This div is hidden.</div>"#),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text(
                    "\u{4e3a}\u{65b9}\u{4fbf}\u{8d77}\u{89c1}\u{ff0c}\u{4e00}\u{4e9b} \
                     _\u{901a}\u{5e38}_ \
                     \u{9700}\u{8981}\u{95ed}\u{5408}\u{6807}\u{7b7e}\u{7684}\u{5143}\u{7d20}\\
                     u{662f}\u{88ab}"
                ),
                bold(vec![text("\u{5141}\u{8bb8}")]),
                text("\u{81ea}\u{95ed}\u{5408}\u{7684}\u{3002}\u{4f8b}\u{5982}\u{ff0c}"),
                code("html! { <div class=\"placeholder\" /> }"),
                text(" \u{8fd9}\u{6837}\u{5199}\u{662f}\u{6709}\u{6548}\u{7684}\u{3002}"),
            ]),]
        ),
        h2(vec![text("Children")]),
        p(vec![text(
            "\u{8f7b}\u{677e}\u{521b}\u{5efa}\u{590d}\u{6742}\u{7684}\u{5d4c}\u{5957} HTML \
             \u{548c} SVG \u{5e03}\u{5c40}\u{ff1a}"
        )]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

let no = 1 + 1 != 2;

html! {
    <div hidden={no}>
        { "This div is NOT hidden." }
    </div>
};"#
        ),
        code_block("rust", r##"<div>This div is NOT hidden.</div>"##),
        h2(vec![text("Classes")]),
        p(vec![text(
            "\u{6709}\u{8bb8}\u{591a}\u{65b9}\u{4fbf}\u{7684}\u{9009}\u{9879}\u{53ef}\u{7528}\\
             u{4e8e}\u{5143}\u{7d20}\u{6307}\u{5b9a} classes\u{ff1a}"
        )]),
        code_block(
            "rust",
            r#"use yew::{html, virtual_dom::AttrValue};

let str_placeholder = "I'm a str!";
let string_placeholder = String::from("I'm a String!");
let attrvalue_placeholder = AttrValue::from("I'm an AttrValue!");

html! {
    <div>
        <input placeholder={str_placeholder} />
        <input placeholder={string_placeholder} />
        <input placeholder={attrvalue_placeholder} />
    </div>
};"#
        ),
        code_block(
            "rust",
            r#"use yew::prelude::*;

let maybe_id = Some("foobar");

html! {
    <div id={maybe_id}></div>
};"#
        ),
        code_block(
            "rust",
            r#"use yew::prelude::*;

let maybe_id = Some("foobar");

html! {
    <div id={maybe_id}></div>
};"#
        ),
        code_block(
            "rust",
            r#"html! {
  <div class={self.classes()}></div>
}"#
        ),
        code_block(
            "rust",
            r#"html! {
  <div class={("class-1", "class-2")}></div>
}"#
        ),
        code_block(
            "rust",
            r#"html! {
  <div class={vec!["class-1", "class-2"]}></div>
}"#
        ),
        h2(vec![text("\u{76d1}\u{542c}\u{5668}")]),
        p(vec![
            text(
                "\u{76d1}\u{542c}\u{5668}\u{5c5e}\u{6027}\u{9700}\u{8981}\u{4f20}\u{9012}\u{4e00}\\
                 \
                 u{4e2a}\u{7531}\u{95ed}\u{5305}\u{5305}\u{88f9}\u{7684} "
            ),
            code("Callback"),
            text(
                "\u{3002}\u{521b}\u{5efa}\u{56de}\u{8c03}\u{7684}\u{65b9}\u{5f0f}\u{53d6}\u{51b3}\\
                 \
                 u{4e8e}\u{4f60}\u{5e0c}\u{671b}\u{4f60}\u{7684}\u{5e94}\u{7528}\u{7a0b}\u{5e8f}\\
                 u{5982}\u{4f55}\u{54cd}\u{5e94}\u{76d1}\u{542c}\u{5668}\u{4e8b}\u{4ef6}\u{ff1a}"
            ),
        ]),
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
                // 处理 Click
            }
        }
    }

    fn view(&self) -> Html {
        // 从组件 link 中创建回调来在组件中处理它
        let click_callback = self.link.callback(|_: ClickEvent| Msg::Click);
        html! {
            <button onclick={click_callback}>
                { "Click me!" }
            </button>
        }
    }
}"#
        ),
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
        // 从 worker 中创建回调来在另一个上下文中处理它
        let click_callback = self.worker.callback(|_: ClickEvent| WorkerMsg::Process);
        html! {
            <button onclick={click_callback}>
                { "Click me!" }
            </button>
        }
    }
}"#
        ),
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
        // 创建一个短暂的回调
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
    ])
);
