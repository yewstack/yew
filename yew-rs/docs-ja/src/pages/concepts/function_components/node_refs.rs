pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            code("ref"),
            text(" 属性を使用して、"),
            code("NodeRef"),
            text(" を HTML 要素にアタッチできます。コールバック内で、"),
            code("ref"),
            text(" がアタッチされた DOM "),
            code("Element"),
            text(" を取得できます。これを使用して、"),
            code("view"),
            text(" ライフサイクルメソッドの外部で DOM を変更したり、"),
            code("<input>"),
            text(" の値を取得したり、JavaScript API を介して直接 DOM と対話したりできます。"),
        ],
        p![text(
            "これは、canvas \
             要素を取得したり、ページの異なる部分にスクロールしたりするのに便利です。",
        )],
        admonition![
            AdmonitionType::Caution,
            None,
            p![
                text(
                    "Yew がレンダリングした DOM \
                     ツリーを手動で変更しないでください。確信が持てない場合は、",
                ),
                code("NodeRef"),
                text(" を読み取り専用アクセスとして扱ってください。"),
            ],
        ],
        h2![text("さらに読む")],
        ul![
            li![link![
                "https://yew-rs-api.web.app/next/yew/functional/fn.use_node_ref.html",
                text("use_node_ref フック"),
            ]],
            li![link![
                "https://github.com/yewstack/yew/tree/master/examples/node_refs",
                code("node_refs"),
                text(" の例"),
            ]],
        ],
    ])
}

crate::doc_page!(
    "ノード参照",
    "/ja/docs/concepts/function-components/node-refs",
    page_content()
);
