pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["Hooks"],
        p!["Hooks 是一類能夠儲存狀態和執行副作用的函數。"],
        p![
            "Yew 提供了一些預先定義的 hooks。您也可以創建自己的 hooks，或發現許多",
            link!("/community/awesome#hooks", "社區製作的 hooks"),
            "。",
        ],
        h2!["Hooks 規則"],
        ol![
            li!["每個 Hook 函數的名稱必須以 ", code("use_"), " 開頭",],
            li!["Hooks 只能在下列位置使用："],
        ],
        ul![
            li!["函數/ Hook 的頂層"],
            li!["函數/ Hook 內的區塊，只要它沒有被分支"],
            li!["函數/ Hook 內頂層 ", code("if"), " 表達式的條件",],
            li!["函數/ Hook 內頂層 ", code("match"), " 表達式的選擇器",],
        ],
        ol![li![
            "每次渲染時，Hooks 必須以相同的順序呼叫。只有在使用 ",
            link!("/zh-Hant/docs/concepts/suspense", "Suspense"),
            " 時才允許提前返回",
        ]],
        p!["這些規則由編譯時或執行時錯誤來執行。"],
        h3!["預定義 Hooks"],
        p!["Yew 提供了以下預定義 Hooks："],
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
            "這些 hooks 的文檔可以在 ",
            link!(
                "https://yew-rs-api.web.app/next/yew/functional/",
                "Yew API 文件",
            ),
            "中找到。",
        ],
        h3!["自訂 Hooks"],
        p!["有些情況下，您可能想要定義自己的 \
            Hooks，以將元件中的可能具有狀態的邏輯封裝到可重複使用的函數中。"],
        h2!["進一步閱讀"],
        ul![li![
            "React 文件中有一個關於 ",
            link!("https://reactjs.org/docs/hooks-intro.html", "React hooks",),
            " 的部分。",
        ]],
    ])
}

crate::doc_page!(
    "Hooks",
    "/zh-Hant/docs/concepts/function-components/hooks",
    page_content()
);
