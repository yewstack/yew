pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("优先级")]),
        p(vec![
            text(
                "框架即将推出的功能和重点的优先级由社区决定。 在 2020 \
                 年春季，我们发送了一份开发者调查，以收集关于项目方向的反馈。 您可以在 ",
            ),
            link(
                "https://github.com/yewstack/yew/wiki/Dev-Survey-%5BSpring-2020%5D",
                vec![text("Yew Wiki")],
            ),
            text(" 中找到调查摘要。"),
        ]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text("所有主要倡议的状态都可以在 Yew Github "),
                link(
                    "https://github.com/yewstack/yew/projects",
                    vec![text("项目看板")],
                ),
                text(" 上跟踪"),
            ])],
        ),
        h2(vec![text("重点")]),
        ol(vec![
            li(vec![text("最受欢迎的功能")]),
            li(vec![text("生产就绪")]),
            li(vec![text("文档")]),
            li(vec![text("痛点")]),
        ]),
        h3(vec![text("最受欢迎的功能")]),
        ol(vec![
            li(vec![link(
                "https://github.com/yewstack/yew/projects/3",
                vec![text("函数组件")],
            )]),
            li(vec![link(
                "https://github.com/yewstack/yew/projects/4",
                vec![text("组件库")],
            )]),
            li(vec![text("更好的状态管理")]),
            li(vec![link(
                "https://github.com/yewstack/yew/projects/5",
                vec![text("服务器端渲染")],
            )]),
        ]),
        h3(vec![text("生产就绪所需的问题")]),
        ul(vec![
            li(vec![text("提高 Yew 测试覆盖率")]),
            li(vec![text("减小二进制文件大小")]),
            li(vec![link(
                "https://github.com/yewstack/yew/issues/5",
                vec![text("性能基准测试")],
            )]),
        ]),
        h3(vec![text("文档")]),
        ul(vec![
            li(vec![text("创建教程")]),
            li(vec![text("简化项目设置")]),
        ]),
        h3(vec![text("痛点")]),
        ul(vec![
            li(vec![link(
                "https://github.com/yewstack/yew/issues/830",
                vec![text("组件样板")],
            )]),
            li(vec![link(
                "https://github.com/yewstack/yew/projects/6",
                vec![text("代理")],
            )]),
        ]),
    ])
}

crate::doc_page!("路线图", "/zh-Hans/docs/more/roadmap", page_content());
