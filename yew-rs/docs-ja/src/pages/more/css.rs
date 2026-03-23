pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![
            text("Yew に CSS サポートを統合する最良の方法についての議論は、こちらで見つけることができます："),
            link("https://github.com/yewstack/yew/issues/533", vec![text("https://github.com/yewstack/yew/issues/533")]),
        ]),
        p(vec![text("ここには、Yew に CSS サポートを統合する最良の方法についての多くの議論が含まれています。")]),
        p(vec![text("現在、私たちが採用している方法は、開発者が最も人気のあるシステムを採用する前に多くのシステムを構築することを奨励することです。")]),
        p(vec![text("コミュニティは現在、プロジェクトにスタイルを追加するためのいくつかのプロジェクトを開発しています。以下はその一部です：")]),
        h4(vec![text("コンポーネントライブラリ")]),
        ul(vec![
            li(vec![link("https://github.com/spielrs/yew_styles", vec![text("yew_styles")]), text(" - JavaScript 依存なしの Yew スタイルフレームワーク。")]),
            li(vec![link("https://github.com/Follpvosten/yew-mdc", vec![text("yew-mdc")]), text(" - マテリアルデザインコンポーネント。")]),
            li(vec![link("https://github.com/AlephAlpha/muicss-yew", vec![text("muicss-yew")]), text(" - MUI CSS コンポーネント。")]),
            li(vec![link("https://github.com/yewstack/yewtify", vec![text("Yewtify")]), text(" – Yew で Vuetify フレームワークの機能を実現。")]),
        ]),
        h4(vec![text("スタイルソリューション")]),
        ul(vec![
            li(vec![link("https://github.com/futursolo/stylist-rs", vec![text("stylist")]), text(" - WebAssembly アプリケーション用の CSS-in-Rust スタイルソリューション。")]),
            li(vec![link("https://github.com/thedodd/trunk/tree/master/examples/yew-tailwindcss", vec![text("tailwind-css")]), text(" - Tailwind ユーティリティクラス。")]),
        ]),
        admonition(AdmonitionType::Important, Some("ドキュメントの改善"), vec![
            p(vec![text("Yew にスタイルを追加するプロジェクトを開発している場合は、このリストに自分を追加する PR を提出してください！")]),
        ]),
    ])
}

crate::doc_page!("CSS", "/ja/docs/more/css", page_content());
