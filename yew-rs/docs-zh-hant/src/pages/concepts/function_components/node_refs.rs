pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            code("ref"),
            " 屬性可以用於將 ",
            code("NodeRef"),
            " 附加到 HTML 元素上。在回呼中，您可以取得 ",
            code("ref"),
            " 附加到的 DOM ",
            code("Element"),
            "。這可以用於在 ",
            code("view"),
            " 生命週期方法之外對 DOM 進行更改，檢索 ",
            code("<input>"),
            " 的值以及透過 javascript API 直接與 DOM 互動。",
        ],
        p!["這對於獲取 canvas 元素或滾動到頁面的不同部分很有用。"],
        admonition!(
            AdmonitionType::Caution,
            None,
            p![
                "不要手動修改 Yew 渲染的 DOM 樹。如果不確定，請將 ",
                code("NodeRef"),
                " 視為唯讀存取。",
            ],
        ),
        h2!["進一步閱讀"],
        ul![
            li![link!(
                "https://yew-rs-api.web.app/next/yew/functional/fn.use_node_ref.html",
                "use_node_ref hook",
            )],
            li![link!(
                "https://github.com/yewstack/yew/tree/master/examples/node_refs",
                code("node_refs"),
                " 範例",
            )],
        ],
    ])
    .with_description("Out-of-band DOM access")
}

crate::doc_page!(
    "節點引用",
    "/zh-Hant/docs/concepts/function-components/node-refs",
    page_content()
);
