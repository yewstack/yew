crate::doc_page!(
    "函数式组件",
    "/zh-Hans/docs/concepts/function-components",
    Content::new(vec![
        p(vec![
            text(
                "函数式组件是普通组件\\
                 u{7684}简化版。它们由一个\\
                 u{63a5}收 props 的函数组成，并通过返\\
                 u{56de}"
            ),
            code("Html"),
            text(
                "来确定应该呈现什么。\\
                 u{57fa}本上，它是一个简化\\
                 u{4e3a}"
            ),
            code("view"),
            text(
                "方法的组件。就其本身\\
                 u{800c}言，这将是相当有限\\
                 u{7684}，因为您只能创建纯\\
                 u{7ec4}件，而这就是 Hook 大展身手的地方。Hook 允许函数组件无需实现"
            ),
            code("Component"),
            text(
                " trait，就可以使用状态\\
                 u{ff08}state）和其他 Yew 功能。"
            ),
        ]),
        h2(vec![text("创建函数式组件")]),
        p(vec![
            text(
                "创建函数式组件的最简\\
                 u{5355}方法是在函数前添加"
            ),
            code("#[function_component]"),
            text("属性。"),
        ]),
        code_block(
            "rust",
            r#"use yew::{function_component, html, Html};

#[function_component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

// Then somewhere else you can use the component inside `html!`
#[function_component]
fn App() -> Html {
    html! { <HelloWorld /> }
}"#
        ),
        h3(vec![text("更多细节")]),
        p(vec![
            text(
                "函数式组件由两部分组\\
                 u{6210}。首先， "
            ),
            code("FunctionProvider"),
            text(" trait 与"),
            code("Component"),
            text(
                " trait 差不多，但它只有一\\
                 u{4e2a}名为"
            ),
            code("run"),
            text("方法。之后是"),
            code("FunctionComponent"),
            text("结构体，它封装了"),
            code("FunctionProvider"),
            text(
                "类型并将其转换为实际\\
                 u{7684}"
            ),
            code("Component"),
            text(" 。 "),
            code("#[function_component]"),
            text("属性本质上只是"),
            code("FunctionProvider"),
            text("并将其暴露在"),
            code("FunctionComponent"),
            text(" 。"),
        ]),
        h3(vec![text("钩子（Hooks）")]),
        p(vec![text(
            "钩子（Hooks）就是让您“\\
             u{94a9}住”组件的状态（state）\\
             u{548c}/或生命周期并执行操\\
             u{4f5c}的函数。 除了 Yew 自带的一些预定义的 Hook。您也可以创建自己的\\
             u{3002}"
        ),]),
    ])
);
