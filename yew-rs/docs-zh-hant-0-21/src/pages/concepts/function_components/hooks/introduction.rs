crate::doc_page!(
    "Hooks",
    "/zh-Hant/docs/concepts/function-components/hooks",
    Content::new(vec![
        h2![text("Hooks")],
        p![text("Hooks 是一類能夠儲存狀態和執行副作用的函數。",)],
        p![
            text("Yew 提供了一些預先定義的 hooks。您也可以創建自己的 hooks，或發現許多"),
            link!("/community/awesome#hooks", text("社區製作的 hooks")),
            text("。"),
        ],
        h2![text("Hooks 規則")],
        ol![
            li![
                text("每個 Hook 函數的名稱必須以 "),
                code("use_"),
                text(" 開頭"),
            ],
            li_blocks![
                p![text("Hooks 只能在下列位置使用：")],
                ul![
                    li![text("函數/ Hook 的頂層")],
                    li![text("函數/ Hook 內的區塊，只要它沒有被分支")],
                    li![
                        text("函數/ Hook 內頂層 "),
                        code("if"),
                        text(" 表達式的條件"),
                    ],
                    li![
                        text("函數/ Hook 內頂層 "),
                        code("match"),
                        text(" 表達式的選擇器"),
                    ],
                ],
            ],
            li![
                text("每次渲染時，Hooks 必須以相同的順序呼叫。只有在使用 "),
                link!("", text("Suspense")),
                text(" 時才允許提前返回"),
            ],
        ],
        p![text("這些規則由編譯時或執行時錯誤來執行。",)],
        h3![text("預定義 Hooks")],
        p![text("Yew 提供了以下預定義 Hooks：")],
        ul![
            li![code("use_state")],
            li![code("use_state_eq")],
            li![code("use_memo")],
            li![code("use_callback")],
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
            text("這些 hooks 的文檔可以在 "),
            link!(
                "https://yew-rs-api.web.app/next/yew/functional/",
                text("Yew API 文件"),
            ),
            text("中找到。"),
        ],
        h3![text("自訂 Hooks")],
        p![
            text(
                "有些情況下，您可能想要定義自己的 \
                 Hooks，以將元件中的可能具有狀態的邏輯封裝到可重複使用的函數中。請參閱",
            ),
            link!(
                "concepts/function-components/hooks/custom-hooks.mdx#defining-custom-hooks",
                text("定義自訂 Hooks"),
            ),
            text("一節了解更多資訊。"),
        ],
        h2![text("進一步閱讀")],
        ul![li![
            text("React 文件中有一個關於 "),
            link!(
                "https://reactjs.org/docs/hooks-intro.html",
                text("React hooks"),
            ),
            text(" 的部分。這些與 Yew 的 hooks 不同，但基本概念相似。"),
        ]],
    ])
);
