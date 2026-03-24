pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("コンポーネントとは？")],
        p![
            text("コンポーネントは Yew の構成要素です。内部状態を管理し、要素を DOM にレンダリングできます。"),
            code("Component"),
            text(" トレイトを実装することでコンポーネントを作成します。"),
        ],
        h2![text("コンポーネントマークアップの作成")],
        p![
            text("Yew は仮想 DOM を使用して要素を DOM にレンダリングします。仮想 DOM ツリーは "),
            code("html!"),
            text(" マクロを使用して構築できます。"),
            code("html!"),
            text(" の構文は HTML に似ていますが、同じではありません。ルールもより厳格です。また、条件付きレンダリングやイテレータを使用したリストのレンダリングなどの強力な機能も提供します。"),
        ],
        admonition!(AdmonitionType::Info, None,
            p![link!(
                "/ja/docs/concepts/html",
                text("html! マクロ、その使用方法、および構文についてさらに詳しく知る"),
            )],
        ),
        h2![text("コンポーネントにデータを渡す")],
        p![
            text("Yew コンポーネントは "),
            italic![text("props")],
            text(" を使用して親コンポーネントと子コンポーネント間で通信します。親コンポーネントは任意のデータを props として子コンポーネントに渡すことができます。Props は HTML 属性に似ていますが、任意の Rust 型を props として渡すことができます。"),
        ],
        admonition!(AdmonitionType::Info, None,
            p![link!(
                "/ja/docs/advanced-topics/struct-components/properties",
                text("props についてさらに詳しく知る"),
            )],
        ),
        admonition!(AdmonitionType::Info, None,
            p![
                text("親/子通信以外の通信には、"),
                link!("/ja/docs/concepts/contexts", text("コンテキスト")),
                text(" を使用してください"),
            ],
        ),
    ])
}

crate::doc_page!(
    "紹介",
    "/ja/docs/advanced-topics/struct-components",
    page_content()
);
