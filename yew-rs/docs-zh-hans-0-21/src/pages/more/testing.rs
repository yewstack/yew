crate::doc_page!(
    "",
    "/zh-Hans/docs/more/testing",
    Content::new(vec![
        h1(vec![text("测试")]),
        h2(vec![text("Rust WebDriving")]),
        p(vec![
            text("使用 Rust 以编程方式驱动 UI 集成测试，"),
            link(
                "https://crates.io/crates/fantoccini",
                vec![text("fantoccini")]
            ),
            text(
                " 是一个推荐的选择。它允许你通过使用 CSS \
                 选择器来查找特定的元素，然后对它们执行特定的操作，例如输入文本，点击按钮，\
                 或等待特定时间以使客户端代码执行（例如等待一个网络请求完成并导致 UI \
                 改变），来测试你的网站。"
            )
        ])
    ])
);
