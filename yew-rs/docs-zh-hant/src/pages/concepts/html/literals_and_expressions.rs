pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("字面量")],
        p![
            text("如果表達式解析為實現了 "),
            code("Display"),
            text(" 的類型，它們將被轉換為字串並插入到DOM 中作為"),
            link!(
                "https://developer.mozilla.org/en-US/docs/Web/API/Text",
                text("Text")
            ),
            text(" 節點。"),
        ],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                text("字串字面量會建立 "),
                code("Text"),
                text(" 節點，瀏覽器將其視為字串。因此，即使表達式包含 "),
                code("<script>"),
                text(" 標籤，您也不會遇到 XSS 等安全性問題，除非您將表達式包裝在 "),
                code("<script>"),
                text(" 區塊中。"),
            ],
        ],
        p![
            text("所有顯示文字都必須用 "),
            code("{}"),
            text(" 區塊括起來，因為文字被視為表達式。這是 Yew 與普通 HTML 語法最大的偏差。"),
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

let text = "lorem ipsum";
html!{
    <>
        <div>{text}</div>
        <div>{"dolor sit"}</div>
        <span>{42}</span>
    </>
};"#,
        ),
        h2![text("表達式")],
        p![
            text("您可以使用 "),
            code("{}"),
            text(" 區塊在 HTML 中插入表達式，只要它們解析為 "),
            code("Html"),
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

let show_link = true;

html! {
  <div>
    {
      if show_link {
        html! {
          <a href="https://example.com">{"Link"}</a>
        }
      } else {
        html! {}
      }
    }
  </div>
};"#,
        ),
        p![text(
            "通常將這些表達式提取到函數或閉包中以優化可讀性是有意義的：",
        )],
        code_block(
            "rust",
            r#"use yew::prelude::*;

let show_link = true;
let maybe_display_link = move || -> Html {
  if show_link {
    html! {
      <a href="https://example.com">{"Link"}</a>
    }
  } else {
    html! {}
  }
};

html! {
     <div>{maybe_display_link()}</div>
};"#,
        ),
    ])
}

crate::doc_page!(
    "字面量與表達式",
    "/zh-Hant/docs/concepts/html/literals-and-expressions",
    page_content()
);
