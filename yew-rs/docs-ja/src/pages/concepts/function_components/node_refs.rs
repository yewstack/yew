pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            code("ref"),
            " 属性を使用して、",
            code("NodeRef"),
            " を HTML 要素にアタッチできます。コールバック内で、",
            code("ref"),
            " がアタッチされた DOM ",
            code("Element"),
            " を取得できます。これを使用して、",
            code("view"),
            " ライフサイクルメソッドの外部で DOM を変更したり、",
            code("<input>"),
            " の値を取得したり、JavaScript API を介して直接 DOM と対話したりできます。",
        ],
        p!["これは、canvas \
            要素を取得したり、ページの異なる部分にスクロールしたりするのに便利です。"],
        admonition![
            AdmonitionType::Caution,
            None,
            p![
                "Yew がレンダリングした DOM \
                 ツリーを手動で変更しないでください。確信が持てない場合は、",
                code("NodeRef"),
                " を読み取り専用アクセスとして扱ってください。",
            ],
        ],
        h2!["さらに読む"],
        ul![
            li![link![
                "https://yew-rs-api.web.app/next/yew/functional/fn.use_node_ref.html",
                "use_node_ref フック",
            ]],
            li![link![
                "https://github.com/yewstack/yew/tree/master/examples/node_refs",
                code("node_refs"),
                " の例",
            ]],
        ],
    ])
    .with_description("Out-of-band DOM access")
}

crate::doc_page!(
    "ノード参照",
    "/ja/docs/concepts/function-components/node-refs",
    page_content()
);
