crate::doc_page!(
    "節點引用",
    "/zh-Hant/docs/concepts/function-components/node-refs",
    Content::new(vec![
        p![
            code("ref"),
            text(" 屬性可以用於將 "),
            code("NodeRef"),
            text(" 附加到 HTML 元素上。在回呼中，您可以取得 "),
            code("ref"),
            text(" 附加到的 DOM "),
            code("Element"),
            text("。這可以用於在 "),
            code("view"),
            text(" 生命週期方法之外對 DOM 進行更改，檢索 "),
            code("<input>"),
            text(" 的值以及透過 javascript API 直接與 DOM 互動。"),
        ],
        p![text(
            "這對於獲取 canvas 元素或滾動到頁面的不同部分很有用。",
        )],
        admonition!(
            AdmonitionType::Caution,
            None,
            p![
                text("不要手動修改 Yew 渲染的 DOM 樹。如果不確定，請將 "),
                code("NodeRef"),
                text(" 視為唯讀存取。"),
            ],
        ),
        h2![text("進一步閱讀")],
        ul![
            li![link!(
                "https://yew-rs-api.web.app/next/yew/functional/fn.use_node_ref.html",
                text("use_node_ref hook"),
            )],
            li![link!(
                "https://github.com/yewstack/yew/tree/master/examples/node_refs",
                text("node_refs 範例"),
            )],
        ],
    ])
);
