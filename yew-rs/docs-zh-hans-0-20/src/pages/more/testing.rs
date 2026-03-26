crate::doc_page!(
    "",
    "/zh-Hans/docs/more/testing",
    Content::new(vec![
        h1!["测试"],
        admonition![
            AdmonitionType::Info,
            None,
            p!["我们正在努力使组件测试变得简单，但目前仍在开发中。"],
            p![
                "对",
                link!("https://github.com/yewstack/yew/issues/1413", "浅渲染",),
                "的支持可以在 GitHub 仓库中找到。",
            ],
        ],
        h2!["快照测试"],
        p![
            "Yew 提供了 ",
            code("yew::tests::layout_tests"),
            " 模块来方便组件的快照测试。",
        ],
        admonition![
            AdmonitionType::Warning,
            Some("贡献"),
            p!["帮助改进快照测试的文档。"],
        ],
        h2!["Rust WebDriving"],
        p![
            "使用 Rust 以编程方式驱动 UI 集成测试，",
            link!("https://crates.io/crates/fantoccini", "fantoccini",),
            " 是一个推荐的选择。它允许你通过使用 CSS \
             选择器来查找特定的元素，然后对它们执行特定的操作，例如输入文本，点击按钮，\
             或等待特定时间以使客户端代码执行（例如等待一个网络请求完成并导致 UI \
             改变），来测试你的网站。",
        ],
    ])
);
