pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["イミュータブルタイプとは？"],
        p![
            "これらのタイプは、インスタンス化はできるが値を変更することはできないタイプです。\
             値を更新するには、新しい値をインスタンス化する必要があります。"
        ],
        h2!["なぜイミュータブルタイプを使用するのですか？"],
        p![
            "React と同様に、プロパティは祖先から子孫に伝播されます。これは、\
             各コンポーネントが更新されるたびにプロパティが存在する必要があることを意味します。\
             したがって、プロパティは理想的には簡単にクローンできるべきです。\
             これを実現するために、通常は ",
            code("Rc"),
            " にラップします。",
        ],
        p!["イミュータブルタイプは、\
            コンポーネント間でプロパティの値を低コストでクローンできるため、\
            プロパティの値を保持するのに最適です。"],
        h2!["一般的なイミュータブルタイプ"],
        p![
            "Yew は ",
            code("implicit-clone"),
            " クレートから以下のイミュータブルタイプの使用を推奨しています：",
        ],
        ul![
            li![
                code("IString"),
                "（Yew では ",
                code("AttrValue"),
                " としてエイリアス化）- ",
                code("String"),
                " の代わりに文字列用",
            ],
            li![
                code("IArray<T>"),
                " - ",
                code("Vec<T>"),
                " の代わりに配列・ベクター用",
            ],
            li![
                code("IMap<K, V>"),
                " - ",
                code("HashMap<K, V>"),
                " の代わりにマップ用",
            ],
        ],
        p![
            "これらのタイプは参照カウント（",
            code("Rc"),
            "）または静的参照のいずれかであり、非常に安価にクローンできます。",
        ],
        h2!["さらに読む"],
        ul![
            li![link![
                "https://github.com/yewstack/yew/tree/master/examples/immutable",
                "イミュータブルの例",
            ]],
            li![link![
                "https://docs.rs/implicit-clone/",
                "Crate ",
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
