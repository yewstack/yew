pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("什麼是元件？")],
        p![
            text("組件是 Yew 的構建塊。它們管理內部狀態並可以將元素渲染到 DOM 中。透過為類型實作 "),
            code("Component"),
            text(" trait 來建立元件。"),
        ],
        h2![text("編寫元件標記")],
        p![
            text("Yew 使用虛擬 DOM 將元素渲染到 DOM 中。虛擬 DOM 樹可以透過使用 "),
            code("html!"),
            text(" 巨集來建構。 "),
            code("html!"),
            text(
                " 使用的語法類似 HTML，但並不相同。規則也更嚴格。\
                 它還提供了條件渲染和使用迭代器渲染清單等超能力。",
            ),
        ],
        admonition!(
            AdmonitionType::Info,
            None,
            p![link!(
                "concepts/html/introduction.mdx",
                text("了解更多關於 "),
                code("html!"),
                text(" 宏，如何使用它以及它的語法"),
            )],
        ),
        h2![text("將資料傳遞給元件")],
        p![
            text("Yew 元件使用 "),
            italic![text("props")],
            text(
                " 在父元件和子元件之間通訊。父元件可以將任何資料作為 props 傳遞給其子元件。 Props \
                 類似於 HTML 屬性，但可以將任何 Rust 類型作為 props 傳遞。",
            ),
        ],
        admonition!(
            AdmonitionType::Info,
            None,
            p![link!(
                "advanced-topics/struct-components/properties.mdx",
                text("了解更多關於 props 的內容"),
            )],
        ),
        admonition!(
            AdmonitionType::Info,
            None,
            p![
                text("對於除了父/子通信之外的其他通信，請使用 "),
                link!("/zh-Hant/docs/concepts/contexts", text("contexts")),
            ],
        ),
    ])
}

crate::doc_page!(
    "簡介",
    "/zh-Hant/docs/advanced-topics/struct-components",
    page_content()
);
