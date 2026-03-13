crate::doc_page!(
    "CSS",
    "/zh-Hant/docs/more/css",
    Content::new(vec![
        p(vec![
            text("有關 CSS 的支援與建議可以在這裡找到： "),
            link(
                "https://github.com/yewstack/yew/issues/533",
                vec![text("https://github.com/yewstack/yew/issues/533")],
            ),
        ]),
        p(vec![text(
            "這裡面包含了大量關於如何將 CSS 支持最好地整合到 Yew 中的討論。"
        )]),
        p(vec![text(
            "目前，我們採取的方式是鼓勵開發者建立多種方案，然後再採用其中最受歡迎的一種。"
        )]),
        p(vec![text(
            "社群目前正在開發一些專案，以便更方便地為專案添加樣式。以下列出了其中一部分："
        )]),
        h4(vec![text("元件庫")]),
        ul(vec![
            li(vec![
                link(
                    "https://github.com/spielrs/yew_styles",
                    vec![text("yew_styles")],
                ),
                text(" - 一個不依賴任何 JavaScript 的 Yew 樣式框架。"),
            ]),
            li(vec![
                link(
                    "https://github.com/Follpvosten/yew-mdc",
                    vec![text("yew-mdc")],
                ),
                text(" - Material Design 元件。"),
            ]),
            li(vec![
                link(
                    "https://github.com/AlephAlpha/muicss-yew",
                    vec![text("muicss-yew")],
                ),
                text(" - MUI CSS 元件。"),
            ]),
            li(vec![
                link("https://github.com/yewstack/yewtify", vec![text("Yewtify")],),
                text(" - 在 Yew 中實現了 Vuetify 框架提供的功能。"),
            ]),
        ]),
        h4(vec![text("樣式方案")]),
        ul(vec![li(vec![
            link(
                "https://github.com/futursolo/stylist-rs",
                vec![text("stylist")],
            ),
            text(" - 一個用於 WebAssembly 應用程式的 CSS-in-Rust 樣式方案。"),
        ])]),
        admonition(
            AdmonitionType::Warning,
            Some("貢獻"),
            vec![p(vec![text(
                "如果你正在開發一個為 Yew 添加樣式的專案，請提交 PR 將自己添加到此列表中！"
            )])]
        ),
    ])
);
