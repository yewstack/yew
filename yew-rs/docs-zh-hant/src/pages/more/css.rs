pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "一個關於如何最好地將CSS 支援整合到Yew 中的討論可以在這裡找到：",
            link!(
                "https://github.com/yewstack/yew/issues/533",
                "https://github.com/yewstack/yew/issues/533",
            ),
        ],
        p!["這裡包含了很多關於如何最好地將 CSS 支援整合到 Yew 中的討論。"],
        p!["目前，我們採用的方法是鼓勵開發者在採用最受歡迎的系統之前建立許多系統。"],
        p!["社區目前正在開發幾個項目，以便為項目添加樣式。以下是其中的一些："],
        h4!["元件庫"],
        ul![
            li![
                link!("https://github.com/spielrs/yew_styles", "yew_styles",),
                " - 沒有任何 JavaScript 依賴的 Yew 樣式框架。",
            ],
            li![
                link!("https://github.com/Follpvosten/yew-mdc", "yew-mdc",),
                " - Material Design 元件。",
            ],
            li![
                link!("https://github.com/AlephAlpha/muicss-yew", "muicss-yew",),
                " - MUI CSS 元件。",
            ],
            li![
                link!("https://github.com/yewstack/yewtify", "Yewtify"),
                " – 在 Yew 中實作 Vuetify 框架所提供的功能。",
            ],
        ],
        h4!["樣式解決方案"],
        ul![
            li![
                link!("https://github.com/futursolo/stylist-rs", "stylist",),
                " - 用於 WebAssembly 應用程式的 CSS-in-Rust 樣式解決方案。",
            ],
            li![
                link!(
                    "https://github.com/thedodd/trunk/tree/master/examples/yew-tailwindcss",
                    "tailwind-css",
                ),
                " - Tailwind 實用類別。",
            ],
        ],
        admonition!(
            AdmonitionType::Important,
            Some("改進文檔"),
            p!["如果您正在開發一個為 Yew 添加樣式的項目，請提交一個 PR 將自己添加到這個列表中！"],
        ),
    ])
}

crate::doc_page!("CSS", "/zh-Hant/docs/more/css", page_content());
