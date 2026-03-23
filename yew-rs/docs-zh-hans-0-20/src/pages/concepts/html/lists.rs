crate::doc_page!(
    "",
    "/zh-Hans/docs/concepts/html/lists",
    Content::new(vec![
        h1(vec![text("列表")]),
        h2(vec![text("Fragments")]),
        p(vec![
            code("html!"),
            text(
                " 宏总是要求一个单一\\
                 u{7684}根节点。为了绕开这\\
                 u{4e2a}限制，把内容包裹在\\
                 u{4e00}个空标签内是有效的\\
                 u{ff1a}"
            ),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

let items = (1..=10).collect::<Vec<_>>();

html! {
    <ul class="item-list">
        { items.iter().collect::<Html>() }
    </ul>
};"#
        ),
        code_block(
            "rust",
            r#"use yew::prelude::*;

let items = (1..=10).collect::<Vec<_>>();

html! {
    <ul class="item-list">
        { for items.iter() }
    </ul>
};"#
        ),
        h2(vec![text("迭代器")]),
        p(vec![text("Yew 支持两种从迭代器构建 html 的语法：")]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

let names = vec!["Sam","Bob","Ray"]

html! {
    <div id="introductions">
        {
            names.into_iter().map(|name| {
                html!{<div key={name}>{ format!("Hello, I'am {}!",name) }</div>}
            }).collect::<Html>()
        }
    </div>
};"#
        ),
        code_block(
            "rust",
            r#"<div id="bob">My name is Bob</div>
<div id="sam">My name is Sam</div>
<div id="rob">My name is rob</div>"#
        ),
    ])
);
