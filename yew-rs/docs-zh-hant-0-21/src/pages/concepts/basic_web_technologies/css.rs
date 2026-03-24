crate::doc_page!(
    "使用 classes! 巨集處理 CSS 類別",
    "/zh-Hant/docs/concepts/basic-web-technologies/css",
    Content::new(vec![
        p![
            text("Yew 並沒有提供原生的 CSS-in-Rust 解決方案，但透過提供程式設計方式與 HTML "),
            code("class"),
            text(" 屬性互動的方式來輔助樣式。"),
        ],
        h2![text("`classes!` 巨集")],
        p![
            code("classes!"),
            text(" 巨集和相關的 "),
            code("Classes"),
            text(" 結構簡化了 HTML 類別的使用："),
        ],
        p![
            text("更多 CSS 相關的內容請參考"),
            link!("", text("此文檔")),
            text("。"),
        ],
        h2![text("內聯樣式")],
        p![
            text("目前 Yew 並沒有提供特殊的輔助工具來處理透過 "),
            code("style"),
            text(" 屬性指定的內聯樣式，但你可以像處理其他 HTML 屬性一樣處理它：",),
        ],
        code_block(
            "rust",
            r#"use yew::{classes, html};

html! {
  <div style="color: red;"></div>
};"#,
        ),
        p![
            text("更多 CSS 相關的內容請參考"),
            link!("", text("此文檔")),
            text("。"),
        ],
    ])
);
