crate::doc_page!(
    "Lists",
    "/zh-Hant/docs/concepts/html/lists",
    Content::new(vec![
        h2(vec![text("Fragments")]),
        p(vec![
            code("html!"),
            text(
                " 巨集裡必須只有一個根結點。為了可以繞過這個限制，將兩個以上的結點，\
                 用空的標籤包裹起來，是合法的：",
            ),
        ]),
        tabs(
            "Valid",
            vec![
                tab(
                    "Valid",
                    "Valid",
                    vec![code_block(
                        "rust",
                        "html! {\n    <>\n        <div></div>\n        <p></p>\n    </>\n}",
                    )],
                ),
                tab(
                    "Invalid",
                    "Invalid",
                    vec![code_block(
                        "rust",
                        "/* error: only one root html element allowed */\n\nhtml! {\n    \
                         <div></div>\n    <p></p>\n}",
                    )],
                ),
            ],
        ),
        h2(vec![text("Iterators")]),
        p(vec![text(
            "Yew 支援兩種不同的方式，從 iterator 建構 html：",
        )]),
        tabs(
            "Syntax Type 1",
            vec![
                tab(
                    "Syntax Type 1",
                    "Syntax Type 1",
                    vec![code_block(
                        "rust",
                        "html! {\n    <ul class=\"item-list\">\n        { \
                         self.props.items.iter().map(renderItem).collect::<Html>() }\n    </ul>\n}",
                    )],
                ),
                tab(
                    "Syntax Type 2",
                    "Syntax Type 2",
                    vec![code_block(
                        "rust",
                        "html! {\n    <ul class=\"item-list\">\n        { for \
                         self.props.items.iter().map(renderItem) }\n    </ul>\n}",
                    )],
                ),
            ],
        ),
    ])
);
