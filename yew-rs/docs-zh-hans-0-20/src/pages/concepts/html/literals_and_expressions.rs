crate::doc_page!(
    "",
    "/zh-Hans/docs/concepts/html/literals-and-expressions",
    Content::new(vec![
        h1(vec![text(
            "\u{5e38}\u{91cf}\u{548c}\u{8868}\u{8fbe}\u{5f0f}"
        )]),
        h2(vec![text("\u{5e38}\u{91cf}")]),
        p(vec![
            text(
                "\u{5982}\u{679c}\u{4e00}\u{4e2a}\u{8868}\u{8fbe}\u{5f0f}\u{7684}\u{7c7b}\u{578b}\\
                 \
                 u{672c}\u{8eab}\u{5b9e}\u{73b0}\u{4e86} "
            ),
            code("Display"),
            text(
                " \u{ff08}\u{4e00}\u{4e2a}\u{6807}\u{51c6}\u{5e93}\u{4e2d}\u{7684} \
                 Trait\u{ff09}\u{ff0c}\u{4ed6}\u{4eec}\u{5c06}\u{4f1a}\u{88ab}\u{8f6c}\u{5316}\\
                 u{4e3a}\u{5b57}\u{7b26}\u{4e32}\u{5e76}\u{4e14}\u{4f5c}\u{4e3a}\u{4e00}\u{4e2a} "
            ),
            link(
                "https://developer.mozilla.org/en-US/docs/Web/API/Text",
                vec![text("Text")]
            ),
            text(" \u{8282}\u{70b9}\u{63d2}\u{5165} DOM \u{4e2d}\u{3002}"),
        ]),
        p(vec![
            text(
                "\u{6240}\u{6709}\u{7684}\u{9700}\u{8981}\u{663e}\u{793a}\u{7684}\u{6587}\u{672c}\\
                 \
                 u{5fc5}\u{987b}\u{88ab} "
            ),
            code("{}"),
            text(
                " \u{5757}\u{5305}\u{542b}\u{ff0c}\u{56e0}\u{4e3a}\u{8fd9}\u{4e9b}\u{6587}\\
                 u{672c}\u{4f1a}\u{88ab}\u{5f53}\u{505a}\u{4e00}\u{4e2a} Rust \
                 \u{8868}\u{8fbe}\u{5f0f}\u{6765}\u{5904}\u{7406}\u{3002}\u{8fd9}\u{4e00}\u{70b9}\\
                 \
                 u{4e0a}\u{ff0c}Yew \u{4e2d}\u{4f7f}\u{7528} HTML \
                 \u{7684}\u{65b9}\u{5f0f}\u{548c}\u{6b63}\u{5e38} HTML \
                 \u{8bed}\u{6cd5}\u{6709}\u{5de8}\u{5927}\u{7684}\u{533a}\u{522b}\u{3002}"
            ),
        ]),
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
        h2(vec![text("\u{8868}\u{8fbe}\u{5f0f}")]),
        p(vec![
            text("\u{4f60}\u{53ef}\u{4ee5}\u{5728} HTML \u{4e2d}\u{4f7f}\u{7528} "),
            code("{}"),
            text(
                " \u{5757}\u{6765}\u{63d2}\u{5165} Rust \
                 \u{8868}\u{8fbe}\u{5f0f}\u{ff0c}\u{53ea}\u{8981}\u{8fd9}\u{4e9b}\u{8868}\u{8fbe}\\
                 \
                 u{5f0f}\u{6700}\u{7ec8}\u{53ef}\u{4ee5}\u{88ab}\u{89e3}\u{6790}\u{6210} "
            ),
            code("Html"),
        ]),
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
        p(vec![text(
            "\u{901a}\u{5e38}\u{6211}\u{4eec}\u{4f1a}\u{628a}\u{8fd9}\u{4e9b}\u{8868}\u{8fbe}\\
             u{5f0f}\u{5199}\u{8fdb}\u{51fd}\u{6570}\u{6216}\u{8005}\u{95ed}\u{5305}\u{4e2d}\\
             u{6765}\u{589e}\u{52a0}\u{53ef}\u{8bfb}\u{6027}\u{ff1a}"
        )]),
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
