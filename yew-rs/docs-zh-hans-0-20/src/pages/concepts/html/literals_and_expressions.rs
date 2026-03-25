crate::doc_page!(
    "",
    "/zh-Hans/docs/concepts/html/literals-and-expressions",
    Content::new(vec![
        h1(vec![text("常量和表达式")]),
        h2![text("常量")],
        p![
            text(
                "如果一个表达式的类型\\
                 u{672c}身实现了 "
            ),
            code("Display"),
            text(
                " （一个标准库中的 Trait），他们将会被转化\\
                 u{4e3a}字符串并且作为一个 "
            ),
            link![
                "https://developer.mozilla.org/en-US/docs/Web/API/Text",
                text("Text")
            ],
            text(" 节点插入 DOM 中。"),
        ],
        p![
            text(
                "所有的需要显示的文本\\
                 u{5fc5}须被 "
            ),
            code("{}"),
            text(
                " 块包含，因为这些文\\
                 u{672c}会被当做一个 Rust 表达式来处理。这一点\\
                 u{4e0a}，Yew 中使用 HTML 的方式和正常 HTML 语法有巨大的区别。"
            ),
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
};"#
        ),
        h2![text("表达式")],
        p![
            text("你可以在 HTML 中使用 "),
            code("{}"),
            text(
                " 块来插入 Rust 表达式，只要这些表达\\
                 u{5f0f}最终可以被解析成 "
            ),
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
};"#
        ),
        p![text(
            "通常我们会把这些表达\\
             u{5f0f}写进函数或者闭包中\\
             u{6765}增加可读性："
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
};"#
        ),
    ])
);
