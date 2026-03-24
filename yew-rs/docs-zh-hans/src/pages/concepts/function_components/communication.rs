pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("父组件向子组件发送消息")],
        p![
            text("将数据作为 "),
            link!(
                "/zh-Hans/docs/concepts/function-components/properties",
                text("props"),
            ),
            text(" 传递，这会导致重新渲染，这是向子组件传递消息的方法。"),
        ],
        h2![text("子组件向父组件发送消息")],
        p![
            text("通过 props 传递一个回调，子组件在事件上可以调用。"),
            link!("callbacks#passing-callbacks-as-props", text("示例")),
        ],
    ])
}

crate::doc_page!(
    "组件之间的通信",
    "/zh-Hans/docs/concepts/function-components/communication",
    page_content()
);
