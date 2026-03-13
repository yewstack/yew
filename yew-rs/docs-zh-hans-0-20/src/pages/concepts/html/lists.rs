crate::doc_page!(
    "",
    "/zh-Hans/docs/concepts/html/lists",
    Content::new(vec![
        h1(vec![text("\u{5217}\u{8868}")]),
        h2(vec![text("Fragments")]),
        p(vec![
            code("html!"),
            text(
                " \u{5b8f}\u{603b}\u{662f}\u{8981}\u{6c42}\u{4e00}\u{4e2a}\u{5355}\u{4e00}\\
                 u{7684}\u{6839}\u{8282}\u{70b9}\u{3002}\u{4e3a}\u{4e86}\u{7ed5}\u{5f00}\u{8fd9}\\
                 u{4e2a}\u{9650}\u{5236}\u{ff0c}\u{628a}\u{5185}\u{5bb9}\u{5305}\u{88f9}\u{5728}\\
                 u{4e00}\u{4e2a}\u{7a7a}\u{6807}\u{7b7e}\u{5185}\u{662f}\u{6709}\u{6548}\u{7684}\\
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
        h2(vec![text("\u{8fed}\u{4ee3}\u{5668}")]),
        p(vec![text(
            "Yew \u{652f}\u{6301}\u{4e24}\u{79cd}\u{4ece}\u{8fed}\u{4ee3}\u{5668}\u{6784}\u{5efa} \
             html \u{7684}\u{8bed}\u{6cd5}\u{ff1a}"
        )]),
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
