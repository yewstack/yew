pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p!["讓我們重新回顧一下之前的標語："],
        blockquote![p![
            "Yew 的核心思想是將可重複使用的 UI 部分所需的所有內容集中在一個地方 - Rust 檔案中。"
        ]],
        p!["我們將透過引入將定義應用程式的邏輯和呈現行為的概念來完善這個陳述：\"元件\"。"],
        h2!["什麼是元件？"],
        p!["組件是 Yew 的構建塊。"],
        p!["它們應當："],
        ul![
            li![
                "以 ",
                link!(
                    "/zh-Hant/docs/concepts/function-components/properties",
                    "Props",
                ),
                " 的形式接受參數",
            ],
            li!["可以擁有自己的狀態"],
            li!["計算使用者可見的 HTML 片段（DOM）"],
        ],
        h2!["Yew 組件的兩種風味"],
        p!["您目前正在閱讀有關函數元件的內容 - 這是在開始使用 Yew \
            時以及在編寫簡單的呈現邏輯時編寫元件的建議方式。"],
        p![
            "還有一種更高級但不太容易訪問的編寫組件的方式 - ",
            link!(
                "/zh-Hant/docs/advanced-topics/struct-components/introduction",
                "結構組件",
            ),
            "。它們允許非常詳細的控制，儘管大多數情況下您不需要那麼詳細的控制。",
        ],
        h2!["建立函數元件"],
        p![
            "若要建立函數元件，請將 ",
            code("#[component]"),
            " 屬性加入到一個函式中。依照慣例，函數的名稱採用 PascalCase，與 ",
            code("html!"),
            " 巨集中的普通 html 元素形成對比。",
        ],
        code_block(
            "rust",
            r#"use yew::{component, html, Html};

#[component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

// 然後在其他地方，您可以在 `html!` 中使用元件
#[component]
fn App() -> Html {
    html! { <HelloWorld /> }
}"#,
        ),
        h2!["組件內部發生了什麼"],
        p![
            "在渲染時，Yew 將建立這些元件的虛擬樹。它將調用每個（函數）元件的 view 函數來計算 DOM \
             的虛擬版本（VDOM），您作為庫用戶將其視為 ",
            code("Html"),
            " 類型。對於上面的範例，這將如下所示：",
        ],
        code_block(
            "xhtml",
            r#"<App>
    <HelloWorld>
        <p>"Hello world"</p>
    </HelloWorld>
</App>"#,
        ),
        p![
            "當需要更新時，Yew 將再次呼叫 view 函數，並將新的虛擬 DOM \
             與其先前的版本進行協調，並僅將新的/更改的/必要的部分傳 播到實際的 \
             DOM。這就是我們所說的 ",
            bold!["渲染"],
            "。",
        ],
        admonition!(
            AdmonitionType::Note,
            None,
            p![
                "實際上，",
                code("Html"),
                " 只是 ",
                code("VNode"),
                " 的別名 - 一個虛擬節點。",
            ],
        ),
    ])
}

crate::doc_page!(
    "函數組件",
    "/zh-Hant/docs/concepts/function-components",
    page_content()
);
