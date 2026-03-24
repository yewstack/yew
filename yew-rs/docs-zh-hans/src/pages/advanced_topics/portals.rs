pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("什么是 Portal？")],
        p![
            text(
                "传送门 (Portal) 提供了一种将子元素渲染到父组件的 DOM 层次结构之外的 DOM \
                 节点的方法。",
            ),
            code("yew::create_portal(child, host)"),
            text(" 返回一个 "),
            code("Html"),
            text(" 值，它将 "),
            code("child"),
            text(" 渲染为 "),
            code("host"),
            text(" 元素的子元素，而不是在其父组件的层次结构下。"),
        ],
        h2![text("用法")],
        p![
            text("传送门的典型用途包括模态对话框和悬停卡片，以及更多技术应用，例如控制元素的 "),
            link![
                "https://developer.mozilla.org/en-US/docs/Web/API/Element/shadowRoot",
                code("shadowRoot"),
            ],
            text(" 的内容，将样式表附加到周围文档的 "),
            code("<head>"),
            text(" 中，以及在 "),
            code("<svg>"),
            text(" 的中央 "),
            code("<defs>"),
            text(" 元素中收集引用的元素。"),
        ],
        p![
            text("请注意，"),
            code("yew::create_portal"),
            text(
                " 是一个低级构建块。库应该使用它来实现更高级的 API，然后应用程序可以使用这些 \
                 API。例如，这里是一个简单的模态对话框，它将其 ",
            ),
            code("children"),
            text(" 渲染到 "),
            code("yew"),
            text(" 之外的一个元素中，该元素由 "),
            code("id=\"modal_host\""),
            text(" 标识。"),
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub children: Html,
}

#[component]
fn Modal(props: &ModalProps) -> Html {
    let modal_host = gloo::utils::document()
        .get_element_by_id("modal_host")
        .expect("Expected to find a #modal_host element");

    create_portal(
        props.children.clone(),
        modal_host.into(),
    )
}"#,
        ),
        h2![text("事件处理")],
        p![text(
            "传送门内部元素上发出的事件遵循虚拟 DOM \
             冒泡。也就是说，如果传送门被渲染为元素的子元素，\
             那么该元素上的事件监听器将捕获从传送门内部分发出的事件，即使传送门将其内容渲染在实际 \
             DOM 中的不相关位置。",
        )],
        p![text(
            "这使开发人员无需关心他们使用的组件是使用传送门实现的还是没有使用传送门实现的。\
             无论如何，其子元素上触发的事件都会冒泡。",
        )],
        p![
            text("已知问题是，从传送门到 "),
            bold![text("关闭")],
            text(
                " 的 shadow root 的事件将被分发两次，一次针对 shadow root \
                 内部的元素，一次针对宿主元素本身。请记住，",
            ),
            bold![text("打开")],
            text(" 的 shadow root 可以正常工作。如果这影响到您，请随时提交一个错误报告。"),
        ],
        h2![text("进一步阅读")],
        ul![li![link![
            "https://github.com/yewstack/yew/tree/master/examples/portals",
            text("传送门示例"),
        ]]],
    ])
}

crate::doc_page!(
    "传送门 (Portals)",
    "/zh-Hans/docs/advanced-topics/portals",
    page_content()
);
