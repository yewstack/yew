pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("DOM ノード")],
        p![text(
            "Yew で DOM ノードを手動で作成または管理する理由はたくさんあります。たとえば、\
             管理されたコンポーネントと競合する可能性のある JS ライブラリとの統合などです。",
        )],
        p![
            code("web-sys"),
            text(" を使用すると、DOM 要素を作成して "),
            code("Node"),
            text(" に変換できます。次に、"),
            code("VRef"),
            text(" を使用して "),
            code("Html"),
            text(" 値として使用できます："),
        ],
        code_block(
            "rust",
            r#"use web_sys::{Element, Node};
use yew::prelude::*;
use gloo::utils::document;

#[component]
fn MyComponent() -> Html {
    // メモ化された関数、一度だけ実行されます
    let node = use_memo(
        (),
        |_| {
            // ドキュメントから div 要素を作成
            let div: Element = document().create_element("div").unwrap();
            // コンテンツ、クラスなどを追加
            div.set_inner_html("Hello, World!");
            // Element を Node に変換
            let node: Node = div.into();
            // その Node を Html 値として返す
            Html::VRef(node)
        },
    );

    // use_memo は Rc ポインタを返すので、参照解除とクローンが必要です
    (*node).clone()
}"#,
        ),
        h2_id!["dynamic-tag-names", text("動的なタグ名")],
        p![
            text(
                "高階コンポーネントを構築する際、\
                 タグ名が静的ではない状況に遭遇することがあります。例えば、",
            ),
            code("Title"),
            text(" コンポーネントがあり、レベル属性に応じて "),
            code("h1"),
            text(" から "),
            code("h6"),
            text(
                " までの任意の内容をレンダリングする場合です。大きなマッチ式を使用する代わりに、\
                 Yew は ",
            ),
            code("@{name}"),
            text(" を使用してタグ名を動的に設定することを許可します。ここで、"),
            code("name"),
            text(" は文字列を返す任意の式です。"),
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
        h2![text("論理値属性")],
        p![text(
            "いくつかのコンテンツ属性（例えば、checked、hidden、\
             required）は論理値属性と呼ばれます。Yew \
             では、論理値属性はブール値に設定する必要があります：",
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
            text("これは次の "),
            bold![text("HTML")],
            text(" と機能的に同等です："),
        ],
        code_block("html", r#"<div hidden>This div is hidden.</div>"#),
        p![text(
            "論理値属性を false \
             に設定することは、その属性を使用しないことと同等です。\
             論理式の値を使用することもできます：",
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
        p![
            text("これは次の "),
            bold![text("HTML")],
            text(" と機能的に同等です："),
        ],
        code_block("html", r#"<div>This div is NOT hidden.</div>"#),
        h2![text("文字列に似た属性")],
        p![text(
            "いくつかの論理値属性に加えて、多くの文字列に似た HTML 属性を扱うことがあります。Yew \
             には、文字列に似た値をコンポーネントに渡すためのいくつかのオプションがあります。",
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
            text(
                "それらはすべて有効ですが、特にクローンを作成する必要がある場合や、\
                 別のコンポーネントに属性として渡す必要がある場合は、Yew のカスタム ",
            ),
            code("AttrValue"),
            text(" を使用することをお勧めします。"),
        ],
        h2![text("HTML 要素のオプション属性")],
        p![text(
            "ほとんどの HTML 属性はオプションの値（Some(x) または \
             None）を使用できます。これにより、\
             属性がオプションとしてマークされている場合にその属性を省略できます。",
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
            text("属性が "),
            code("None"),
            text(" に設定されている場合、その属性は DOM に設定されません。"),
        ],
        h2![text("関連例")],
        ul![li![link![
            "https://github.com/yewstack/yew/tree/master/examples/inner_html",
            text("インライン HTML"),
        ]]],
    ])
}

crate::doc_page!("要素", "/ja/docs/concepts/html/elements", page_content());
