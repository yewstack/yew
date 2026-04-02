crate::doc_page!(
    "Lists",
    "/zh-Hant/docs/concepts/html/lists",
    Content::new(vec![
        h2!["Fragments"],
        p![
            code("html!"),
            " 巨集裡必須只有一個根結點。為了可以繞過這個限制，將兩個以上的結點，\
             用空的標籤包裹起來，是合法的：",
        ],
        tabs![
            "valid",
            tab![
                "Valid",
                "valid",
                code_block(
                    "rust",
                    r#"html! {
    <>
        <div></div>
        <p></p>
    </>
}"#,
                ),
            ],
            tab![
                "Invalid",
                "invalid",
                code_block(
                    "rust",
                    r#"/* error: only one root html element allowed */

html! {
    <div></div>
    <p></p>
}"#,
                ),
            ],
        ],
        h2!["Iterators"],
        p!["Yew 支援兩種不同的方式，從 iterator 建構 html："],
        tabs![
            "syntax-type-1",
            tab![
                "Syntax Type 1",
                "syntax-type-1",
                code_block(
                    "rust",
                    r#"html! {
    <ul class="item-list">
        { self.props.items.iter().map(renderItem).collect::<Html>() }
    </ul>
}"#,
                ),
            ],
            tab![
                "Syntax Type 2",
                "syntax-type-2",
                code_block(
                    "rust",
                    r#"html! {
    <ul class="item-list">
        { for self.props.items.iter().map(renderItem) }
    </ul>
}"#,
                ),
            ],
        ],
    ])
);
