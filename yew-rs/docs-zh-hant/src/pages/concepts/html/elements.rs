pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("DOM 節點")]),
        p(vec![text(
            "在 Yew 中手動建立或管理 DOM 節點的原因有很多，例如與可能與受管理元件衝突的 JS \
             庫整合。",
        )]),
        p(vec![
            text("使用 "),
            code("web-sys"),
            text("，您可以建立 DOM 元素並將其轉換為 "),
            code("Node"),
            text(" - 然後可以使用 "),
            code("VRef"),
            text(" 將其用作 "),
            code("Html"),
            text(" 值："),
        ]),
        code_block(
            "rust",
            r#"use web_sys::{Element, Node};
use yew::prelude::*;
use gloo::utils::document;

#[component]
fn MyComponent() -> Html {
    // 帶記憶能力的函數，只會執行一次
    let node = use_memo(
        (),
        |_| {
            // 從文件中建立一個 div 元素
            let div: Element = document().create_element("div").unwrap();
            // 新增內容、類別等
            div.set_inner_html("Hello, World!");
            // 將 Element 轉換為 Node
            let node: Node = div.into();
            // 將該 Node 作為 Html 值傳回
            Html::VRef(node)
        },
    );

    // use_memo 回傳的是 Rc 指針，所以我們需要解引用和克隆
    (*node).clone()
}
"#,
        ),
        h2(vec![text("動態標籤名")]),
        p(vec![
            text(
                "在建立高階元件時，您可能會發現自己處於一個標籤名不是靜態的情況。例如，\
                 您可能有一個 ",
            ),
            code("Title"),
            text(" 元件，根據等級屬性可以渲染從 "),
            code("h1"),
            text(" 到 "),
            code("h6"),
            text(" 的任何內容。而不是使用一個大的匹配表達式，Yew 允許您動態設定標籤名，使用 "),
            code("@{name}"),
            text("，其中 "),
            code("name"),
            text(" 可以是傳回字串的任何表達式。"),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

let level = 5;
let text = "Hello World!".to_owned();

html! {
    <@{format!("h{}", level)} class="title">{ text }</@>
};"#,
        ),
        h2(vec![text("邏輯值屬性")]),
        p(vec![text(
            "一些內容屬性（例如 checked、hidden、required）被稱為邏輯值屬性。在 Yew \
             中，邏輯值屬性需要設定為布林值：",
        )]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

html! {
    <div hidden=true>
        { "This div is hidden." }
    </div>
};"#,
        ),
        p(vec![
            text("這與以下的 "),
            bold(vec![text("HTML")]),
            text(" 功能上是等價的："),
        ]),
        code_block("html", r#"<div hidden>This div is hidden.</div>"#),
        p(vec![text(
            "將邏輯值屬性設為 false 等效於不使用該屬性；可以使用邏輯表達式的值：",
        )]),
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
        p(vec![
            text("這與以下 "),
            bold(vec![text("HTML")]),
            text(" 結果等價："),
        ]),
        code_block("html", r#"<div>This div is NOT hidden.</div>"#),
        h2(vec![text("類似字串的屬性")]),
        p(vec![text(
            "除了一些邏輯值屬性，您可能會處理許多類似字串的 HTML 屬性，Yew \
             有幾種選項可以將類似字串的值傳遞給元件。",
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
};"#,
        ),
        p(vec![
            text("它們都是有效的，"),
            bold(vec![text("但")]),
            text("我們鼓勵您更傾向於使用 Yew 的自訂 "),
            code("AttrValue"),
            text("，特別是如果您需要複製或將它們作為屬性傳遞給另一個元件。"),
        ]),
        h2(vec![text("HTML 元素的可選屬性")]),
        p(vec![text(
            "大多數 HTML 屬性可以使用可選值（Some(x) 或 \
             None）。這使我們可以在屬性被標記為可選時省略該屬性。",
        )]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

let maybe_id = Some("foobar");

html! {
    <div id={maybe_id}></div>
};"#,
        ),
        p(vec![
            text("如果屬性設為 "),
            code("None"),
            text("，則該屬性將不會在 DOM 中設定。"),
        ]),
        h2(vec![text("相關範例")]),
        ul(vec![li(vec![link(
            "https://github.com/yewstack/yew/tree/master/examples/inner_html",
            vec![text("內嵌 HTML")],
        )])]),
    ])
}

crate::doc_page!(
    "元素",
    "/zh-Hant/docs/concepts/html/elements",
    page_content()
);
