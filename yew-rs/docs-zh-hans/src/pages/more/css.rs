pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            text("一个关于如何最好地将 CSS 支持集成到 Yew 中的讨论可以在这里找到："),
            link!(
                "https://github.com/yewstack/yew/issues/533",
                text("https://github.com/yewstack/yew/issues/533"),
            ),
        ],
        p![text(
            "这里包含了很多关于如何最好地将 CSS 支持集成到 Yew 中的讨论。",
        )],
        p![text(
            "目前，我们采用的方法是鼓励开发者在采用最流行的系统之前构建许多系统。",
        )],
        p![text(
            "社区目前正在开发几个项目，以便为项目添加样式。以下是其中的一些：",
        )],
        h4![text("组件库")],
        ul![
            li![
                link!("https://github.com/spielrs/yew_styles", text("yew_styles"),),
                text(" - 一个没有任何 JavaScript 依赖的 Yew 样式框架。"),
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
                text(" – 在 Yew 中实现 Vuetify 框架提供的功能。"),
            ],
        ],
        h4![text("样式解决方案")],
        ul![
            li![
                link!("https://github.com/futursolo/stylist-rs", text("stylist"),),
                text(" - 用于 WebAssembly 应用程序的 CSS-in-Rust 样式解决方案。"),
            ],
            li![
                link!(
                    "https://github.com/thedodd/trunk/tree/master/examples/yew-tailwindcss",
                    text("tailwind-css"),
                ),
                text(" - Tailwind 实用类。"),
            ],
        ],
        admonition!(
            AdmonitionType::Important,
            Some("改进文档"),
            p![text(
                "如果您正在开发一个为 Yew 添加样式的项目，请提交一个 PR 将自己添加到这个列表中！",
            )],
        ),
    ])
}

crate::doc_page!("CSS", "/zh-Hans/docs/more/css", page_content());
