pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![
            text("Yew 仓库包含许多"),
            link(
                "https://github.com/yewstack/yew/tree/master/examples",
                vec![text("示例")],
            ),
            text(
                "（维护状态各异）。 我们建议浏览它们以了解如何使用框架的不同功能。 \
                 我们也欢迎拉取请求和问题，以便在它们不可避免地被忽略并需要一些帮助时使用。",
            ),
        ]),
        p(vec![
            text("有关更多详细信息，包括示例列表，请参阅"),
            link(
                "https://github.com/yewstack/yew/tree/master/examples#yew-examples",
                vec![text("README")],
            ),
            text("。"),
        ]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![text(
                "大多数示例都有一个可以在 https://examples.yew.rs/< example_name > \
                 找到的在线部署。\n在各自的子文件夹中的 README \
                 页面上点击它们的徽章以导航到在线演示。",
            )])],
        ),
    ])
}

crate::doc_page!(
    "示例",
    "/zh-Hans/docs/getting-started/examples",
    page_content()
);
