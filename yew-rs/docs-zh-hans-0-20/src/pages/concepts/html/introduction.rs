crate::doc_page!(
    "",
    "/zh-Hans/docs/concepts/html",
    Content::new(vec![
        h1(vec![text("使用 html! 宏")]),
        p(vec![
            code("html!"),
            text(
                " 宏允许你为组件编写\\
                 u{58f0}明式的 HTML 和 SVG。如果你使用过 React 的 JSX，将会感觉到非常熟\\
                 u{6089}。"
            ),
        ]),
        p(vec![bold(vec![text("重要提示")])]),
        ol(vec![
            li(vec![
                code("html!"),
                text(
                    " 宏调用中只能有一个\\
                     u{6839}节点"
                ),
            ]),
            li(vec![
                text("空的 "),
                code("html! {}"),
                text(
                    " 宏调用是有效的但不\\
                     u{4f1a}渲染任何内容"
                ),
            ]),
            li(vec![
                text(
                    "常量必须始终被引号\\
                     u{62ec}起来并被包含在大\\
                     u{62ec}号里："
                ),
                code("html! { \"Hello, World\" }"),
            ]),
        ]),
    ])
);
