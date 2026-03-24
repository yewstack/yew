pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            text("Yew に CSS サポートを統合する最良の方法についての議論は、こちらで見つけることができます："),
            link!("https://github.com/yewstack/yew/issues/533", text("https://github.com/yewstack/yew/issues/533")),
        ],
        p![text("ここには、Yew に CSS サポートを統合する最良の方法についての多くの議論が含まれています。")],
        p![text("現在、私たちが採用している方法は、開発者が最も人気のあるシステムを採用する前に多くのシステムを構築することを奨励することです。")],
        p![text("コミュニティは現在、プロジェクトにスタイルを追加するためのいくつかのプロジェクトを開発しています。以下はその一部です：")],
        h4![text("コンポーネントライブラリ")],
        ul![
            li![link!("https://github.com/spielrs/yew_styles", text("yew_styles")), text(" - JavaScript 依存なしの Yew スタイルフレームワーク。")],
            li![link!("https://github.com/Follpvosten/yew-mdc", text("yew-mdc")), text(" - マテリアルデザインコンポーネント。")],
            li![link!("https://github.com/AlephAlpha/muicss-yew", text("muicss-yew")), text(" - MUI CSS コンポーネント。")],
            li![link!("https://github.com/yewstack/yewtify", text("Yewtify")), text(" – Yew で Vuetify フレームワークの機能を実現。")],
        ],
        h4![text("スタイルソリューション")],
        ul![
            li![link!("https://github.com/futursolo/stylist-rs", text("stylist")), text(" - WebAssembly アプリケーション用の CSS-in-Rust スタイルソリューション。")],
            li![link!("https://github.com/thedodd/trunk/tree/master/examples/yew-tailwindcss", text("tailwind-css")), text(" - Tailwind ユーティリティクラス。")],
        ],
        admonition![AdmonitionType::Important, Some("ドキュメントの改善"),
            p![text("Yew にスタイルを追加するプロジェクトを開発している場合は、このリストに自分を追加する PR を提出してください！")],
        ],
    ])
}

crate::doc_page!("CSS", "/ja/docs/more/css", page_content());
