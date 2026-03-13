pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("Hooks")]),
        p(vec![text("Hooks 是一類能夠儲存狀態和執行副作用的函數。")]),
        p(vec![
            text("Yew 提供了一些預先定義的 hooks。您也可以創建自己的 hooks，或發現許多"),
            link("/community/awesome#hooks", vec![text("社區製作的 hooks")]),
            text("。"),
        ]),
        h2(vec![text("Hooks 規則")]),
        ol(vec![
            li(vec![
                text("每個 Hook 函數的名稱必須以 "),
                code("use_"),
                text(" 開頭"),
            ]),
            li(vec![text("Hooks 只能在下列位置使用：")]),
        ]),
        ul(vec![
            li(vec![text("函數/ Hook 的頂層")]),
            li(vec![text("函數/ Hook 內的區塊，只要它沒有被分支")]),
            li(vec![
                text("函數/ Hook 內頂層 "),
                code("if"),
                text(" 表達式的條件"),
            ]),
            li(vec![
                text("函數/ Hook 內頂層 "),
                code("match"),
                text(" 表達式的選擇器"),
            ]),
        ]),
        ol(vec![li(vec![
            text("每次渲染時，Hooks 必須以相同的順序呼叫。只有在使用 "),
            link("/zh-Hant/docs/concepts/suspense", vec![text("Suspense")]),
            text(" 時才允許提前返回"),
        ])]),
        p(vec![text("這些規則由編譯時或執行時錯誤來執行。")]),
        h3(vec![text("預定義 Hooks")]),
        p(vec![text("Yew 提供了以下預定義 Hooks：")]),
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
            text("這些 hooks 的文檔可以在 "),
            link(
                "https://yew-rs-api.web.app/next/yew/functional/",
                vec![text("Yew API 文件")],
            ),
            text("中找到。"),
        ]),
        h3(vec![text("自訂 Hooks")]),
        p(vec![text(
            "有些情況下，您可能想要定義自己的 \
             Hooks，以將元件中的可能具有狀態的邏輯封裝到可重複使用的函數中。",
        )]),
        h2(vec![text("進一步閱讀")]),
        ul(vec![li(vec![
            text("React 文件中有一個關於 "),
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
    "/zh-Hant/docs/concepts/function-components/hooks",
    page_content()
);
