pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["リテラル"],
        p![
            "式が ",
            code("Display"),
            " を実装する型に解決される場合、それらは文字列に変換され、",
            link![
                "https://developer.mozilla.org/en-US/docs/Web/API/Text",
                "Text",
            ],
            " ノードとしてDOMに挿入されます。",
        ],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                "文字列リテラルは ",
                code("Text"),
                " ノードを作成し、ブラウザはそれを文字列として扱います。そのため、式に ",
                code("<script>"),
                " タグが含まれていても、式を ",
                code("<script>"),
                " ブロックでラップしない限り、XSS \
                 などのセキュリティ問題に遭遇することはありません。",
            ],
        ],
        p![
            "すべての表示テキストは式と見なされるため、",
            code("{}"),
            " ブロックで囲む必要があります。これは、Yew と通常の HTML 構文の最大の違いです。",
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
        h2!["式"],
        p![
            code("{}"),
            " ブロックを使用して、HTML 内に式を挿入できます。それらが ",
            code("Html"),
            " に解決される限り。",
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
        p!["通常、これらの式を関数やクロージャに抽出して、\
            可読性を最適化することが意味があります："],
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
    "リテラルと式",
    "/ja/docs/concepts/html/literals-and-expressions",
    page_content()
);
