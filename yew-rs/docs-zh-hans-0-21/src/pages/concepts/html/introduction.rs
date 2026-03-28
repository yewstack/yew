crate::doc_page!(
    "",
    "/zh-Hans/docs/concepts/html",
    Content::new(vec![
        h1!["使用 html! 宏"],
        p![
            code("html!"),
            " 宏允许你为组件编写声明式的 HTML 和 SVG。如果你使用过 React 的 \
             JSX，将会感觉到非常熟悉。"
        ],
        p![bold!["重要提示"]],
        ol![
            li![code("html!"), " 宏调用中只能有一个根节点"],
            li![
                "空的 ",
                code("html! {}"),
                " 宏调用是有效的但不会渲染任何内容"
            ],
            li![
                "常量必须始终被引号括起来并被包含在大括号里：",
                code("html! { \"Hello, World\" }")
            ]
        ]
    ])
    .with_description("The procedural macro for generating HTML and SVG")
);
