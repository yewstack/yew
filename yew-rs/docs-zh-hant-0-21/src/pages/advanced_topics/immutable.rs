crate::doc_page!(
    "不可變型別",
    "/zh-Hant/docs/advanced-topics/immutable",
    Content::new(vec![
        h2![text("什麼是不可變型別？")],
        p![text(
            "這些類型是您可以實例化但永遠不會更改值的類型。為了更新值，您必須實例化一個新值。"
        )],
        h2![text("為什麼使用不可變型別？")],
        p![
            text(
                "與 React 一樣，屬性是從祖先傳播到子代的。這意味著屬性在每個元件更新時必須存在。\
                 這就是為什麼屬性應該——理想情況下——很容易克隆。為了實現這一點，\
                 我們通常將事物包裝在 "
            ),
            code("Rc"),
            text(" 中。"),
        ],
        p![text(
            "不可變類型非常適合保存屬性的值，因為它們可以在從組件傳遞到組件時以很低的成本克隆。"
        )],
        h2![text("常見的不可變型別")],
        p![
            text("Yew 推薦使用來自 "),
            code("implicit-clone"),
            text(" crate 的以下不可變型別："),
        ],
        ul![
            li![
                code("IString"),
                text("（在 Yew 中別名為 "),
                code("AttrValue"),
                text("）- 用於字串而不是 "),
                code("String"),
            ],
            li![
                code("IArray<T>"),
                text(" - 用於陣列/向量而不是 "),
                code("Vec<T>"),
            ],
            li![
                code("IMap<K, V>"),
                text(" - 用於映射而不是 "),
                code("HashMap<K, V>"),
            ],
        ],
        p![
            text("這些型別是引用計數（"),
            code("Rc"),
            text("）或靜態引用，使它們的克隆成本非常低。"),
        ],
        h2![text("進一步閱讀")],
        ul![
            li![link!(
                "https://github.com/yewstack/yew/tree/master/examples/immutable",
                text("不可變範例"),
            )],
            li![link!(
                "https://docs.rs/implicit-clone/",
                text("Crate implicit-clone"),
            )],
        ],
    ])
);
