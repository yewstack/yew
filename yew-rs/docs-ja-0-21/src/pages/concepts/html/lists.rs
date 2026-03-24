crate::doc_page!(
    "Lists",
    "/ja/docs/concepts/html/lists",
    Content::new(vec![
        h2![text("フラグメント")],
        p![
            code("html!"),
            text(
                "マクロは常にルートノードが 1 \
                 つであることを要求します。この制限のために、\
                 空のタグを使って内容をラップすると良いでしょう。"
            ),
        ],
        p![text("Valid:")],
        code_block(
            "rust",
            r#"html! {
    <>
        <div></div>
        <p></p>
    </>
}"#
        ),
        p![text("Invalid:")],
        code_block(
            "rust",
            r#"/* error: only one root html element allowed */

html! {
    <div></div>
    <p></p>
}"#
        ),
        h2![text("イテレータ")],
        p![text(
            "Yew はイテレータから HTML をビルドするのに 2 つの方法をサポートしています。"
        )],
        p![text("Syntax Type 1:")],
        code_block(
            "rust",
            r#"html! {
    <ul class="item-list">
        { self.props.items.iter().map(renderItem).collect::<Html>() }
    </ul>
}"#
        ),
        p![text("Syntax Type 2:")],
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
