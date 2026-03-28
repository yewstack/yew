pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["什么是组件？"],
        p![
            "组件是 Yew 的构建块。它们管理内部状态并可以将元素渲染到 DOM 中。通过为类型实现 ",
            code("Component"),
            " trait 来创建组件。",
        ],
        h2!["编写组件标记"],
        p![
            "Yew 使用虚拟 DOM 将元素渲染到 DOM 中。虚拟 DOM 树可以通过使用 ",
            code("html!"),
            " 宏来构建。",
            code("html!"),
            " 使用的语法类似于 HTML，但并不相同。规则也更严格。\
             它还提供了诸如条件渲染和使用迭代器渲染列表等超能力。",
        ],
        admonition![
            AdmonitionType::Info,
            None,
            p![link!(
                "concepts/html/introduction.mdx",
                "了解更多关于 ",
                code("html!"),
                " 宏，如何使用它以及它的语法",
            )],
        ],
        h2!["将数据传递给组件"],
        p![
            "Yew 组件使用 ",
            italic!["props"],
            " 在父组件和子组件之间通信。父组件可以将任何数据作为 props 传递给其子组件。Props \
             类似于 HTML 属性，但可以将任何 Rust 类型作为 props 传递。",
        ],
        admonition![
            AdmonitionType::Info,
            None,
            p![link!(
                "advanced-topics/struct-components/properties.mdx",
                "了解更多关于 props 的内容",
            )],
        ],
        admonition![
            AdmonitionType::Info,
            None,
            p![
                "对于除了父/子通信之外的其他通信，请使用 ",
                doc_link!(crate::pages::concepts::contexts, "contexts"),
            ],
        ],
    ])
    .with_description("Components in Yew")
}

crate::doc_page!(
    "简介",
    "/zh-Hans/docs/advanced-topics/struct-components",
    page_content()
);
