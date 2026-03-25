crate::doc_page!(
    "",
    "/zh-Hans/docs/more/css",
    Content::new(vec![
        h1![text("CSS")],
        p![
            text("对适当的 CSS 支持的提案可以在这里找到："),
            link!(
                "https://github.com/yewstack/yew/issues/533",
                text("https://github.com/yewstack/yew/issues/533"),
            ),
        ],
        p![text(
            "这里面包含了大量关于如何将 CSS 支持最好地集成到 Yew 中的讨论。"
        )],
        p![text(
            "目前，我们采取的方式是鼓励开发者构建多种方案，然后再采用其中最受欢迎的一种。"
        )],
        p![text(
            "社区目前正在开发一些项目，以便更方便地为项目添加样式。以下列出了其中一部分："
        )],
        h4![text("组件库")],
        ul![
            li![
                link!("https://github.com/spielrs/yew_styles", text("yew_styles"),),
                text(" - 一个不依赖任何 JavaScript 的 Yew 样式框架。"),
            ],
            li![
                link!("https://github.com/Follpvosten/yew-mdc", text("yew-mdc"),),
                text(" - Material Design 组件。"),
            ],
            li![
                link!(
                    "https://github.com/AlephAlpha/muicss-yew",
                    text("muicss-yew"),
                ),
                text(" - MUI CSS 组件。"),
            ],
            li![
                link!("https://github.com/yewstack/yewtify", text("Yewtify")),
                text(" - 在 Yew 中实现了 Vuetify 框架提供的功能。"),
            ],
        ],
        h4![text("样式方案")],
        ul![li![
            link!("https://github.com/futursolo/stylist-rs", text("stylist"),),
            text(" - 一个用于 WebAssembly 应用程序的 CSS-in-Rust 样式方案。"),
        ]],
        admonition![
            AdmonitionType::Warning,
            Some("贡献"),
            p![text(
                "如果你正在开发一个为 Yew 添加样式的项目，请提交 PR 将自己添加到此列表中！"
            )],
        ],
    ])
);
