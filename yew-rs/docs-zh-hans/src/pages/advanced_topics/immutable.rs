pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("什么是不可变类型？")]),
        p(vec![text(
            "这些类型是您可以实例化但永远不会更改值的类型。为了更新值，您必须实例化一个新值。",
        )]),
        h2(vec![text("为什么使用不可变类型？")]),
        p(vec![
            text(
                "与 React 一样，属性是从祖先传播到子代的。这意味着属性在每个组件更新时必须存在。\
                 这就是为什么属性应该——理想情况下——很容易克隆。为了实现这一点，\
                 我们通常将事物包装在 ",
            ),
            code("Rc"),
            text(" 中。"),
        ]),
        p(vec![text(
            "不可变类型非常适合保存属性的值，因为它们可以在从组件传递到组件时以很低的成本克隆。",
        )]),
        h2(vec![text("常见的不可变类型")]),
        p(vec![
            text("Yew 推荐使用来自 "),
            code("implicit-clone"),
            text(" crate 的以下不可变类型："),
        ]),
        ul(vec![
            li(vec![
                code("IString"),
                text("（在 Yew 中别名为 "),
                code("AttrValue"),
                text("）- 用于字符串而不是 "),
                code("String"),
            ]),
            li(vec![
                code("IArray<T>"),
                text(" - 用于数组/向量而不是 "),
                code("Vec<T>"),
            ]),
            li(vec![
                code("IMap<K, V>"),
                text(" - 用于映射而不是 "),
                code("HashMap<K, V>"),
            ]),
        ]),
        p(vec![
            text("这些类型是引用计数（"),
            code("Rc"),
            text("）或静态引用，使它们的克隆成本非常低。"),
        ]),
        h2(vec![text("进一步阅读")]),
        ul(vec![
            li(vec![link(
                "https://github.com/yewstack/yew/tree/master/examples/immutable",
                vec![text("不可变示例")],
            )]),
            li(vec![link(
                "https://docs.rs/implicit-clone/",
                vec![text("Crate "), code("implicit-clone")],
            )]),
        ]),
    ])
}

crate::doc_page!(
    "不可变类型",
    "/zh-Hans/docs/advanced-topics/immutable",
    page_content()
);
