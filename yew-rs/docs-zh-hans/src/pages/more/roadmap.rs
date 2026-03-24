pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("优先级")],
        p![
            text(
                "框架即将推出的功能和重点的优先级由社区决定。 在 2020 \
                 年春季，我们发送了一份开发者调查，以收集关于项目方向的反馈。 您可以在 ",
            ),
            link!(
                "https://github.com/yewstack/yew/wiki/Dev-Survey-%5BSpring-2020%5D",
                text("Yew Wiki"),
            ),
            text(" 中找到调查摘要。"),
        ],
        admonition!(
            AdmonitionType::Note,
            None,
            p![
                text("所有主要倡议的状态都可以在 Yew Github "),
                link!("https://github.com/yewstack/yew/projects", text("项目看板"),),
                text(" 上跟踪"),
            ],
        ),
        h2![text("重点")],
        ol![
            li![text("最受欢迎的功能")],
            li![text("生产就绪")],
            li![text("文档")],
            li![text("痛点")],
        ],
        h3![text("最受欢迎的功能")],
        ol![
            li![link!(
                "https://github.com/yewstack/yew/projects/3",
                text("函数组件"),
            )],
            li![link!(
                "https://github.com/yewstack/yew/projects/4",
                text("组件库"),
            )],
            li![text("更好的状态管理")],
            li![link!(
                "https://github.com/yewstack/yew/projects/5",
                text("服务器端渲染"),
            )],
        ],
        h3![text("生产就绪所需的问题")],
        ul![
            li![text("提高 Yew 测试覆盖率")],
            li![text("减小二进制文件大小")],
            li![link!(
                "https://github.com/yewstack/yew/issues/5",
                text("性能基准测试"),
            )],
        ],
        h3![text("文档")],
        ul![li![text("创建教程")], li![text("简化项目设置")],],
        h3![text("痛点")],
        ul![
            li![link!(
                "https://github.com/yewstack/yew/issues/830",
                text("组件样板"),
            )],
            li![link!(
                "https://github.com/yewstack/yew/projects/6",
                text("代理"),
            )],
        ],
    ])
}

crate::doc_page!("路线图", "/zh-Hans/docs/more/roadmap", page_content());
