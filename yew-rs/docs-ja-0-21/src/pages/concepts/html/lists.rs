crate::doc_page!(
    "Lists",
    "/ja/docs/concepts/html/lists",
    Content::new(vec![
        h2!["フラグメント"],
        p![
            code("html!"),
            "マクロは常にルートノードが 1 \
             つであることを要求します。この制限のために、\
             空のタグを使って内容をラップすると良いでしょう。",
        ],
        p!["Valid:"],
        code_block(
            "rust",
            r#"html! {
    <>
        <div></div>
        <p></p>
    </>
}"#
        ),
        p!["Invalid:"],
        code_block(
            "rust",
            r#"/* error: only one root html element allowed */

html! {
    <div></div>
    <p></p>
}"#
        ),
        h2!["イテレータ"],
        p!["Yew はイテレータから HTML をビルドするのに 2 つの方法をサポートしています。"],
        p!["Syntax Type 1:"],
        code_block(
            "rust",
            r#"html! {
    <ul class="item-list">
        { self.props.items.iter().map(renderItem).collect::<Html>() }
    </ul>
}"#
        ),
        p!["Syntax Type 2:"],
        code_block(
            "rust",
            r#"html! {
    <ul class="item-list">
        { for self.props.items.iter().map(renderItem) }
    </ul>
}"#
        ),
    ])
);
