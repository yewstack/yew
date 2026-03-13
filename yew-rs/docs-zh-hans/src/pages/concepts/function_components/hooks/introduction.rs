pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("Hooks")]),
        p(vec![text("Hooks 是一类能够存储状态和执行副作用的函数。")]),
        p(vec![
            text("Yew 提供了一些预定义的 hooks。您也可以创建自己的 hooks，或者发现许多"),
            link("/community/awesome#hooks", vec![text("社区制作的 hooks")]),
            text("。"),
        ]),
        h2(vec![text("Hooks 规则")]),
        ol(vec![
            li(vec![
                text("每个 Hook 函数的名称必须以 "),
                code("use_"),
                text(" 开头"),
            ]),
            li_blocks(vec![
                p(vec![text("Hooks 只能在以下位置使用：")]),
                ul(vec![
                    li(vec![text("函数/ Hook 的顶层")]),
                    li(vec![text("函数/ Hook 内的块，只要它没有被分支")]),
                    li(vec![
                        text("函数/ Hook 内顶层 "),
                        code("if"),
                        text(" 表达式的条件"),
                    ]),
                    li(vec![
                        text("函数/ Hook 内顶层 "),
                        code("match"),
                        text(" 表达式的选择器"),
                    ]),
                ]),
            ]),
            li(vec![
                text("每次渲染时，Hooks 必须以相同的顺序调用。只有在使用 "),
                link("/zh-Hans/docs/concepts/suspense", vec![text("Suspense")]),
                text(" 时才允许提前返回"),
            ]),
        ]),
        p(vec![text("这些规则由编译时或运行时错误来执行。")]),
        h3(vec![text("预定义 Hooks")]),
        p(vec![text("Yew 提供了以下预定义 Hooks：")]),
        ul(vec![
            li(vec![code("use_state")]),
            li(vec![code("use_state_eq")]),
            li(vec![code("use_memo")]),
            li(vec![code("use_callback")]),
            li(vec![code("use_ref")]),
            li(vec![code("use_mut_ref")]),
            li(vec![code("use_node_ref")]),
            li(vec![code("use_reducer")]),
            li(vec![code("use_reducer_eq")]),
            li(vec![code("use_effect")]),
            li(vec![code("use_effect_with")]),
            li(vec![code("use_context")]),
            li(vec![code("use_force_update")]),
        ]),
        p(vec![
            text("这些 hooks 的文档可以在 "),
            link(
                "https://yew-rs-api.web.app/next/yew/functional/",
                vec![text("Yew API 文档")],
            ),
            text("中找到。"),
        ]),
        h3(vec![text("自定义 Hooks")]),
        p(vec![text(
            "有些情况下，您可能希望定义自己的 \
             Hooks，以将组件中的可能具有状态的逻辑封装到可重用的函数中。",
        )]),
        h2(vec![text("进一步阅读")]),
        ul(vec![li(vec![
            text("React 文档中有一个关于 "),
            link(
                "https://reactjs.org/docs/hooks-intro.html",
                vec![text("React hooks")],
            ),
            text(" 的部分。"),
        ])]),
    ])
}

crate::doc_page!(
    "Hooks",
    "/zh-Hans/docs/concepts/function-components/hooks",
    page_content()
);
