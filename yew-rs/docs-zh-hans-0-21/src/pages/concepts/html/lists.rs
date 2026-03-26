crate::doc_page!(
    "",
    "/zh-Hans/docs/concepts/html/lists",
    Content::new(vec![
        h1!["列表"],
        h2!["Fragments"],
        p![
            code("html!"),
            " 宏总是要求一个单一的根节点。为了绕开这个限制，把内容包裹在一个空标签内是有效的："
        ],
        code_block(
            "rust",
            r#"html! {
    <>
        <div></div>
        <p></p>
    </>
}"#
        ),
        code_block(
            "rust",
            r#"/* 错误：只允许一个 html 根元素 */

html! {
    <div></div>
    <p></p>
}"#
        ),
        h2!["迭代器"],
        p!["Yew 支持两种从迭代器构建 html 的语法："],
        code_block(
            "rust",
            r#"html! {
    <ul class="item-list">
        { self.props.items.iter().map(renderItem).collect::<Html>() }
    </ul>
}"#
        ),
        code_block(
            "rust",
            r#"html! {
    <ul class="item-list">
        { for self.props.items.iter().map(renderItem) }
    </ul>
}"#
        )
    ])
);
