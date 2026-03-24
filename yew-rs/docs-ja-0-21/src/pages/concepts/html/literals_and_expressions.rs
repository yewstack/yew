crate::doc_page!(
    "Literals and Expressions",
    "/ja/docs/concepts/html/literals-and-expressions",
    Content::new(vec![
        h2![text("リテラル")],
        p![
            text("式が"),
            code("Display"),
            text("を実装した型を解決する場合、文字列に変換されて DOM に"),
            link![
                "https://developer.mozilla.org/en-US/docs/Web/API/Text",
                text("Text")
            ],
            text("ノードとして挿入されます。"),
        ],
        p![
            text("テキストは式として処理されるため、全ての表示される内容は"),
            code("{}"),
            text(
                "ブロックによって囲まれる必要があります。これは Yew のアプリと通常の HTML \
                 の構文で最も異なる点です。"
            ),
        ],
        code_block(
            "rust",
            r#"let text = "lorem ipsum";
html!{
    <>
        <div>{text}</div>
        <div>{"dolor sit"}</div>
        <span>{42}</span>
    </>
}"#
        ),
        h2![text("式")],
        p![
            text("HTML に"),
            code("{}"),
            text("ブロックを使って式を挿入することができます。"),
        ],
        code_block(
            "rust",
            r#"html! {
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
}"#
        ),
        p![text(
            "式を関数やクロージャに分離するのはコードの可読性の観点から有効なことがあります。"
        )],
        code_block(
            "rust",
            r#"let show_link = true;
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
}"#
        ),
    ])
);
