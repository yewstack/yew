pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("イミュータブルタイプとは？")],
        p![text(
            "これらのタイプは、インスタンス化はできるが値を変更することはできないタイプです。\
             値を更新するには、新しい値をインスタンス化する必要があります。",
        )],
        h2![text("なぜイミュータブルタイプを使用するのですか？")],
        p![
            text(
                "React と同様に、プロパティは祖先から子孫に伝播されます。これは、\
                 各コンポーネントが更新されるたびにプロパティが存在する必要があることを意味します。\
                 したがって、プロパティは理想的には簡単にクローンできるべきです。\
                 これを実現するために、通常は ",
            ),
            code("Rc"),
            text(" にラップします。"),
        ],
        p![text(
            "イミュータブルタイプは、\
             コンポーネント間でプロパティの値を低コストでクローンできるため、\
             プロパティの値を保持するのに最適です。",
        )],
        h2![text("一般的なイミュータブルタイプ")],
        p![
            text("Yew は "),
            code("implicit-clone"),
            text(" クレートから以下のイミュータブルタイプの使用を推奨しています："),
        ],
        ul![
            li![
                code("IString"),
                text("（Yew では "),
                code("AttrValue"),
                text(" としてエイリアス化）- "),
                code("String"),
                text(" の代わりに文字列用"),
            ],
            li![
                code("IArray<T>"),
                text(" - "),
                code("Vec<T>"),
                text(" の代わりに配列・ベクター用"),
            ],
            li![
                code("IMap<K, V>"),
                text(" - "),
                code("HashMap<K, V>"),
                text(" の代わりにマップ用"),
            ],
        ],
        p![
            text("これらのタイプは参照カウント（"),
            code("Rc"),
            text("）または静的参照のいずれかであり、非常に安価にクローンできます。"),
        ],
        h2![text("さらに読む")],
        ul![
            li![link![
                "https://github.com/yewstack/yew/tree/master/examples/immutable",
                text("イミュータブルの例"),
            ]],
            li![link![
                "https://docs.rs/implicit-clone/",
                text("Crate "),
                code("implicit-clone"),
            ]],
        ],
    ])
}

crate::doc_page!(
    "イミュータブルタイプ",
    "/ja/docs/advanced-topics/immutable",
    page_content()
);
