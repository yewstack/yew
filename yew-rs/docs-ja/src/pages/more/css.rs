pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "Yew に CSS サポートを統合する最良の方法についての議論は、こちらで見つけることができます：",
            link!("https://github.com/yewstack/yew/issues/533", "https://github.com/yewstack/yew/issues/533"),
        ],
        p!["ここには、Yew に CSS サポートを統合する最良の方法についての多くの議論が含まれています。"],
        p!["現在、私たちが採用している方法は、開発者が最も人気のあるシステムを採用する前に多くのシステムを構築することを奨励することです。"],
        p!["コミュニティは現在、プロジェクトにスタイルを追加するためのいくつかのプロジェクトを開発しています。以下はその一部です："],
        h4!["コンポーネントライブラリ"],
        ul![
            li![link!("https://github.com/spielrs/yew_styles", "yew_styles"), " - JavaScript 依存なしの Yew スタイルフレームワーク。"],
            li![link!("https://github.com/Follpvosten/yew-mdc", "yew-mdc"), " - マテリアルデザインコンポーネント。"],
            li![link!("https://github.com/AlephAlpha/muicss-yew", "muicss-yew"), " - MUI CSS コンポーネント。"],
            li![link!("https://github.com/yewstack/yewtify", "Yewtify"), " – Yew で Vuetify フレームワークの機能を実現。"],
        ],
        h4!["スタイルソリューション"],
        ul![
            li![link!("https://github.com/futursolo/stylist-rs", "stylist"), " - WebAssembly アプリケーション用の CSS-in-Rust スタイルソリューション。"],
            li![link!("https://github.com/thedodd/trunk/tree/master/examples/yew-tailwindcss", "tailwind-css"), " - Tailwind ユーティリティクラス。"],
        ],
        admonition![AdmonitionType::Important, Some("ドキュメントの改善"),
            p!["Yew にスタイルを追加するプロジェクトを開発している場合は、このリストに自分を追加する PR を提出してください！"],
        ],
    ])
}

crate::doc_page!("CSS", "/ja/docs/more/css", page_content());
