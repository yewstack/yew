pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("Hooks")],
        p![text("Hooks 是一类能够存储状态和执行副作用的函数。")],
        p![
            text("Yew 提供了一些预定义的 hooks。您也可以创建自己的 hooks，或者发现许多"),
            link!("/community/awesome#hooks", text("社区制作的 hooks")),
            text("。"),
        ],
        h2![text("Hooks 规则")],
        ol![
            li![
                text("每个 Hook 函数的名称必须以 "),
                code("use_"),
                text(" 开头"),
            ],
            li_blocks![
                p![text("Hooks 只能在以下位置使用：")],
                ul![
                    li![text("函数/ Hook 的顶层")],
                    li![text("函数/ Hook 内的块，只要它没有被分支")],
                    li![
                        text("函数/ Hook 内顶层 "),
                        code("if"),
                        text(" 表达式的条件"),
                    ],
                    li![
                        text("函数/ Hook 内顶层 "),
                        code("match"),
                        text(" 表达式的选择器"),
                    ],
                ],
            ],
            li![
                text("每次渲染时，Hooks 必须以相同的顺序调用。只有在使用 "),
                link!("/zh-Hans/docs/concepts/suspense", text("Suspense")),
                text(" 时才允许提前返回"),
            ],
        ],
        p![text("这些规则由编译时或运行时错误来执行。")],
        h3![text("预定义 Hooks")],
        p![text("Yew 提供了以下预定义 Hooks：")],
        ul![
            li![code("use_state")],
            li![code("use_state_eq")],
            li![code("use_memo")],
            li![code("use_callback")],
            li![code("use_ref")],
            li![code("use_mut_ref")],
            li![code("use_node_ref")],
            li![code("use_reducer")],
            li![code("use_reducer_eq")],
            li![code("use_effect")],
            li![code("use_effect_with")],
            li![code("use_context")],
            li![code("use_force_update")],
        ],
        p![
            text("这些 hooks 的文档可以在 "),
            link!(
                "https://yew-rs-api.web.app/next/yew/functional/",
                text("Yew API 文档"),
            ),
            text("中找到。"),
        ],
        h3![text("自定义 Hooks")],
        p![text(
            "有些情况下，您可能希望定义自己的 \
             Hooks，以将组件中的可能具有状态的逻辑封装到可重用的函数中。",
        )],
        h2![text("进一步阅读")],
        ul![li![
            text("React 文档中有一个关于 "),
            link!(
                "https://reactjs.org/docs/hooks-intro.html",
                text("React hooks"),
            ),
            text(" 的部分。"),
        ]],
    ])
}

crate::doc_page!(
    "Hooks",
    "/zh-Hans/docs/concepts/function-components/hooks",
    page_content()
);
