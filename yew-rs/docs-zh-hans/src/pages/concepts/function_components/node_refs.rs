pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![
            code("ref"),
            text(" 属性可以用于将 "),
            code("NodeRef"),
            text(" 附加到 HTML 元素上。在回调中，您可以获取 "),
            code("ref"),
            text(" 附加到的 DOM "),
            code("Element"),
            text("。这可以用于在 "),
            code("view"),
            text(" 生命周期方法之外对 DOM 进行更改，检索 "),
            code("<input>"),
            text(" 的值以及通过 javascript API 直接与 DOM 交互。"),
        ]),
        p(vec![text(
            "这对于获取 canvas 元素或滚动到页面的不同部分很有用。",
        )]),
        admonition(
            AdmonitionType::Caution,
            None,
            vec![p(vec![
                text("不要手动修改 Yew 渲染的 DOM 树。如果不确定，请将 "),
                code("NodeRef"),
                text(" 视为只读访问。"),
            ])],
        ),
        h2(vec![text("进一步阅读")]),
        ul(vec![
            li(vec![link(
                "https://yew-rs-api.web.app/next/yew/functional/fn.use_node_ref.html",
                vec![text("use_node_ref hook")],
            )]),
            li(vec![link(
                "https://github.com/yewstack/yew/tree/master/examples/node_refs",
                vec![code("node_refs"), text(" 示例")],
            )]),
        ]),
    ])
}

crate::doc_page!(
    "节点引用",
    "/zh-Hans/docs/concepts/function-components/node-refs",
    page_content()
);
