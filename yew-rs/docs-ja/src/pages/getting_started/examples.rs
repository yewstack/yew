pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![
            text("Yew リポジトリには多くの"),
            link(
                "https://github.com/yewstack/yew/tree/master/examples",
                vec![text("例")],
            ),
            text(
                "（メンテナンス状態はさまざま）があります。 \
                 フレームワークのさまざまな機能を理解するために、\
                 それらを参照することをお勧めします。 \
                 無視されがちで助けが必要な場合に備えて、プルリクエストや問題も歓迎します \
                 \u{2665}\u{FE0F}。",
            ),
        ]),
        p(vec![
            text("詳細については、"),
            link(
                "https://github.com/yewstack/yew/tree/master/examples#yew-examples",
                vec![text("README")],
            ),
            text(" を参照してください。"),
        ]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![text(
                "ほとんどの例には、https://examples.yew.rs/< example_name > \
                 で見つけることができるオンラインデプロイがあります。 それぞれのサブフォルダーの \
                 README ページでバッジをクリックして、オンラインデモに移動します。",
            )])],
        ),
    ])
}

crate::doc_page!("例", "/ja/docs/getting-started/examples", page_content());
