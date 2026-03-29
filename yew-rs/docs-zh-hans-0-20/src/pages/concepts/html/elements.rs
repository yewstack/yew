crate::doc_page!(
    "",
    "/zh-Hans/docs/concepts/html/elements",
    Content::new(vec![
        h1!["元素"],
        h2!["标签结构"],
        p![
            "元素标签必须是自闭合\\
                 u{7684} ",
            code("<... />"),
            "，或是每个标签都有一\\
                 u{4e2a}对应的闭合标签。",
        ],
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
        admonition![
            AdmonitionType::Note,
            None,
            p![
                "为方便起见，一些 _通常_ 需要闭合标签的元素\\
                     u{662f}被",
                bold!["允许"],
                "自闭合的。例如，",
                code("html! { <div class=\"placeholder\" /> }"),
                " 这样写是有效的。",
            ],
        ],
        h2!["Children"],
        p!["轻松创建复杂的嵌套 HTML 和 SVG 布局："],
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
        h2!["Classes"],
        p!["有许多方便的选项可用\\
             u{4e8e}元素指定 classes："],
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
        h2!["监听器"],
        p![
            "监听器属性需要传递一\\
                 u{4e2a}由闭包包裹的 ",
            code("Callback"),
            "。创建回调的方式取决\\
                 u{4e8e}你希望你的应用程序\\
                 u{5982}何响应监听器事件：",
        ],
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
    .with_description("Both HTML and SVG elements are supported")
);
