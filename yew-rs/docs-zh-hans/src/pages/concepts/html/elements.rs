pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("DOM 节点")],
        p![text(
            "在 Yew 中手动创建或管理 DOM 节点的原因有很多，比如与可能与受管理组件冲突的 JS \
             库集成。",
        )],
        p![
            text("使用 "),
            code("web-sys"),
            text("，您可以创建 DOM 元素并将其转换为 "),
            code("Node"),
            text(" - 然后可以使用 "),
            code("VRef"),
            text(" 将其用作 "),
            code("Html"),
            text(" 值："),
        ],
        code_block(
            "rust",
            r#"use web_sys::{Element, Node};
use yew::prelude::*;
use gloo::utils::document;

#[component]
fn MyComponent() -> Html {
    // 带记忆能力的函数，只会执行一次
    let node = use_memo(
        (),
        |_| {
            // 从文档中创建一个 div 元素
            let div: Element = document().create_element("div").unwrap();
            // 添加内容、类等
            div.set_inner_html("Hello, World!");
            // 将 Element 转换为 Node
            let node: Node = div.into();
            // 将该 Node 作为 Html 值返回
            Html::VRef(node)
        },
    );

    // use_memo 返回的是 Rc 指针，所以我们需要解引用和克隆
    (*node).clone()
}"#,
        ),
        h2![text("动态标签名")],
        p![
            text(
                "在构建高阶组件时，您可能会发现自己处于一个标签名不是静态的情况。例如，\
                 您可能有一个 ",
            ),
            code("Title"),
            text(" 组件，根据级别属性可以渲染从 "),
            code("h1"),
            text(" 到 "),
            code("h6"),
            text(" 的任何内容。而不是使用一个大的匹配表达式，Yew 允许您动态设置标签名，使用 "),
            code("@{name}"),
            text("，其中 "),
            code("name"),
            text(" 可以是返回字符串的任何表达式。"),
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

let level = 5;
let text = "Hello World!".to_owned();

html! {
    <@{format!("h{}", level)} class="title">{ text }</@>
};"#,
        ),
        h2![text("逻辑值属性")],
        p![text(
            "一些内容属性（例如 checked、hidden、required）被称为逻辑值属性。在 Yew \
             中，逻辑值属性需要设置为布尔值：",
        )],
        code_block(
            "rust",
            r#"use yew::prelude::*;

html! {
    <div hidden=true>
        { "This div is hidden." }
    </div>
};"#,
        ),
        p![
            text("这与以下的 "),
            bold![text("HTML")],
            text(" 功能上是等价的："),
        ],
        code_block("html", r#"<div hidden>This div is hidden.</div>"#),
        p![text(
            "将逻辑值属性设置为 false 等效于不使用该属性；可以使用逻辑表达式的值：",
        )],
        code_block(
            "rust",
            r#"use yew::prelude::*;

let no = 1 + 1 != 2;

html! {
    <div hidden={no}>
        { "This div is NOT hidden." }
    </div>
};"#,
        ),
        p![text("这与以下 "), bold![text("HTML")], text(" 结果等价："),],
        code_block("html", r#"<div>This div is NOT hidden.</div>"#),
        h2![text("类似字符串的属性")],
        p![text(
            "除了一些逻辑值属性，您可能会处理很多类似字符串的 HTML 属性，Yew \
             有几种选项可以将类似字符串的值传递给组件。",
        )],
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
};"#,
        ),
        p![
            text("它们都是有效的，"),
            bold![text("但")],
            text("我们鼓励您更倾向于使用 Yew 的自定义 "),
            code("AttrValue"),
            text("，特别是如果您需要克隆或将它们作为属性传递给另一个组件。"),
        ],
        h2![text("HTML 元素的可选属性")],
        p![text(
            "大多数 HTML 属性可以使用可选值（Some(x) 或 \
             None）。这使我们可以在属性被标记为可选时省略该属性。",
        )],
        code_block(
            "rust",
            r#"use yew::prelude::*;

let maybe_id = Some("foobar");

html! {
    <div id={maybe_id}></div>
};"#,
        ),
        p![
            text("如果属性设置为 "),
            code("None"),
            text("，则该属性将不会在 DOM 中设置。"),
        ],
        h2![text("相关示例")],
        ul![li![link!(
            "https://github.com/yewstack/yew/tree/master/examples/inner_html",
            text("内嵌 HTML"),
        )]],
    ])
}

crate::doc_page!(
    "元素",
    "/zh-Hans/docs/concepts/html/elements",
    page_content()
);
