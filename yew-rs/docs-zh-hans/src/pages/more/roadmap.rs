pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["优先级"],
        p![
            "框架即将推出的功能和重点的优先级由社区决定。 在 2020 \
             年春季，我们发送了一份开发者调查，以收集关于项目方向的反馈。 您可以在 ",
            link!(
                "https://github.com/yewstack/yew/wiki/Dev-Survey-%5BSpring-2020%5D",
                "Yew Wiki",
            ),
            " 中找到调查摘要。",
        ],
        admonition!(
            AdmonitionType::Note,
            None,
            p![
                "所有主要倡议的状态都可以在 Yew Github ",
                link!("https://github.com/yewstack/yew/projects", "项目看板",),
                " 上跟踪",
            ],
        ),
        h2!["重点"],
        ol![
            li!["最受欢迎的功能"],
            li!["生产就绪"],
            li!["文档"],
            li!["痛点"],
        ],
        h3!["最受欢迎的功能"],
        ol![
            li![link!(
                "https://github.com/yewstack/yew/projects/3",
                "函数组件",
            )],
            li![link!(
                "https://github.com/yewstack/yew/projects/4",
                "组件库",
            )],
            li!["更好的状态管理"],
            li![link!(
                "https://github.com/yewstack/yew/projects/5",
                "服务器端渲染",
            )],
        ],
        h3!["生产就绪所需的问题"],
        ul![
            li!["提高 Yew 测试覆盖率"],
            li!["减小二进制文件大小"],
            li![link!(
                "https://github.com/yewstack/yew/issues/5",
                "性能基准测试",
            )],
        ],
        h3!["文档"],
        ul![li!["创建教程"], li!["简化项目设置"],],
        h3!["痛点"],
        ul![
            li![link!(
                "https://github.com/yewstack/yew/issues/830",
                "组件样板",
            )],
            li![link!("https://github.com/yewstack/yew/projects/6", "代理",)],
        ],
    ])
    .with_description("The planned feature roadmap for the Yew framework")
}

crate::doc_page!("路线图", "/zh-Hans/docs/more/roadmap", page_content());
