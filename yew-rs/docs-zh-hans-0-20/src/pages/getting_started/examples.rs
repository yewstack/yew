crate::doc_page!(
    "",
    "/zh-Hans/docs/getting-started/examples",
    Content::new(vec![
        h1(vec![text("通过例子学习")]),
        p![text(
            "Yew 的 github \
             项目中就包含了各种各样的示例（这些项目在不同程度的维护中）。我们建议仔细地学习它们, \
             了解如何使用不同的框架特性. 我们在书中有纰漏和错误的时候也欢迎 pull-requests 和提交 \
             issues ♥️"
        )],
        ul![
            li![link![
                "https://github.com/yewstack/yew/tree/v0.14.0/examples/std_web/todomvc",
                text("**Todo App（代办事项）\\(stdweb\\)**"),
            ]],
            li![link![
                "https://github.com/yewstack/yew/tree/v0.14.0/examples/web_sys/todomvc",
                text("**Todo App（代办事项）\\(web_sys\\)**"),
            ]],
            li![link![
                "https://github.com/yewstack/yew/tree/v0.14.0/examples/custom_components",
                text("**Custom Components（自定义 Component 组件）**"),
            ]],
            li![link![
                "https://github.com/yewstack/yew/tree/v0.14.0/examples/std_web/multi_thread",
                text("**Multi-threading \\(Agents\\)（多线程 Agents）\\(stdweb\\)**"),
            ]],
            li![link![
                "https://github.com/yewstack/yew/tree/v0.14.0/examples/web_sys/multi_thread",
                text("**Multi-threading \\(Agents\\)（多线程 Agents）\\(web_sys\\)**"),
            ]],
            li![link![
                "https://github.com/yewstack/yew/tree/v0.14.0/examples/timer",
                text("**Timer Service（计时器）**"),
            ]],
            li![link![
                "https://github.com/yewstack/yew/tree/v0.14.0/examples/nested_list",
                text("**Nested Components（嵌套 Component 组件）**"),
            ]],
        ],
    ])
);
