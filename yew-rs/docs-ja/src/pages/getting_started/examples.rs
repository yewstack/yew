pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "Yew リポジトリには多くの",
            link!("https://github.com/yewstack/yew/tree/master/examples", "例",),
            "（メンテナンス状態はさまざま）があります。 \
             フレームワークのさまざまな機能を理解するために、それらを参照することをお勧めします。 \
             無視されがちで助けが必要な場合に備えて、プルリクエストや問題も歓迎します ♥️。",
        ],
        p![
            "詳細については、",
            link!(
                "https://github.com/yewstack/yew/tree/master/examples#yew-examples",
                "README",
            ),
            " を参照してください。",
        ],
        admonition!(
            AdmonitionType::Note,
            None,
            p![
                "ほとんどの例には、https://examples.yew.rs/< example_name > \
                 で見つけることができるオンラインデプロイがあります。 それぞれのサブフォルダーの \
                 README ページでバッジをクリックして、オンラインデモに移動します。"
            ],
        ),
    ])
}

crate::doc_page!("例", "/ja/docs/getting-started/examples", page_content());
