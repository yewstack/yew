pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["迭代器"],
        p!["從迭代器建立 HTML 有 3 種方法："],
        tabs![
            "`for` 迴圈",
            tab![
                "`for` 迴圈",
                "`for` 迴圈",
                p!["主要方法是使用 for 迴圈，與 Rust 中已有的 for 迴圈相同，但有 2 個關鍵區別："],
                ol![
                    li![
                        "與標準 for 迴圈不能傳回任何內容不同，",
                        code("html!"),
                        " 中的 for 迴圈會被轉換為節點清單；",
                    ],
                    li![
                        "發散運算式，即 ",
                        code("break"),
                        "、",
                        code("continue"),
                        " 在 ",
                        code("html!"),
                        " 中的 for 迴圈主體內是不允許的。",
                    ],
                ],
                code_block(
                    "rust",
                    r#"use yew::prelude::*;

html! {
    for i in 0 .. 10 {
        <span>{i}</span>
    }
};"#,
                ),
            ],
            tab![
                "`for` 區塊",
                "`for` 區塊",
                p![
                    "另一種方法是使用 ",
                    code("for"),
                    " 關鍵字，這不是原生的 Rust 語法，而是由 HTML \
                     巨集用於輸出顯示迭代器所需的程式碼。當迭代器已經計算好，\
                     只需要將其傳遞給巨集時，這種方法比第一種更好。",
                ],
                code_block(
                    "rust",
                    r#"use yew::prelude::*;

let items = (1..=10).collect::<Vec<_>>();

html! {
    <ul class="item-list">
        { for items.iter() }
    </ul>
};"#,
                ),
            ],
            tab![
                "`collect` 方法",
                "`collect` 方法",
                p![
                    "最後一種方法是在迭代器的最終轉換上呼叫 ",
                    code("collect::<Html>()"),
                    "，它傳回一個 Yew 可以顯示的清單。",
                ],
                code_block(
                    "rust",
                    r#"use yew::prelude::*;

let items = (1..=10).collect::<Vec<_>>();

html! {
    <ul class="item-list">
        { items.iter().collect::<Html>() }
    </ul>
};"#,
                ),
            ],
        ],
        h2!["鍵 (Key) 列表"],
        p![
            "鍵 (Key) 列表是一個最佳化的列表，其中",
            bold!["所有"],
            "子元素都有鍵。 ",
            code("key"),
            " 是 Yew 提供的一個特殊屬性，它為 HTML 元素或元件提供一個唯一標識符，用於 Yew \
             內部的最佳化。",
        ],
        admonition![
            AdmonitionType::Caution,
            None,
            p![
                "Key 只需要在每個清單中是唯一的，與 HTML ",
                code("id"),
                " 的全域唯一性相反。它不應該依賴於列表的順序。",
            ],
        ],
        p!["始終建議為清單新增按鍵 (key)。"],
        p![
            "可以透過將唯一的 ",
            code("String"),
            "、",
            code("str"),
            " 或整數傳遞給特殊的 ",
            code("key"),
            " 屬性來新增鍵：",
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

let names = vec!["Sam","Bob","Ray"]

html! {
    <div id="introductions">
        {
            names.into_iter().map(|name| {
                html!{<div key={name}>{ format!("Hello, I'am {}!",name) }</div>}
            }).collect::<Html>()
        }
    </div>
};
"#,
        ),
        h3!["效能優化"],
        p![
            "我們有一個",
            link!(
                "https://github.com/yewstack/yew/tree/master/examples/keyed_list",
                "帶有鍵 (keys) 的列表範例",
            ),
            "可以讓你測試效能上的改進，這裡有一個簡單的測試流程：",
        ],
        ol![
            li![
                "進入",
                link!("https://examples.yew.rs/keyed_list", "線上示範"),
            ],
            li!["新增 500 個元素"],
            li!["停用鍵"],
            li!["反轉列表"],
            li!["看 \"最後一次渲染花費了 Xms\"（在撰寫本文時，大約為 60ms）"],
            li!["啟用鍵"],
            li!["再次反轉列表"],
            li!["看 \"最後一次渲染花費了 Xms\"（在撰寫本文時，大約為 30ms）"],
        ],
        p!["截至撰寫本文時，對於 500 個組件，速度提高了 2 倍。"],
        h3!["原理解釋"],
        p![
            "通常，當你迭代時，只需要在每個列表項目上添加一個鍵，資料的順序可能會改變。 \
             在重新渲染清單時，它用於加速協調過程。"
        ],
        p![
            "如果沒有鍵，假設你迭代 ",
            code("[\"bob\", \"sam\", \"rob\"]"),
            "，最終得到的 HTML 如下：",
        ],
        code_block(
            "html",
            r#"<div id="bob">My name is Bob</div>
<div id="sam">My name is Sam</div>
<div id="rob">My name is rob</div>"#,
        ),
        p![
            "然後在下一次渲染時，如果你的清單更改為 ",
            code("[\"bob\", \"rob\"]"),
            "，Yew 可以刪除 id=\"rob\" 的元素，並將 id=\"sam\" 更新為 id=\"rob\"。",
        ],
        p![
            "如果你為每個元素添加了一個鍵，初始HTML 將保持不變，但在使用修改後的列表",
            code("[\"bob\", \"rob\"]"),
            " 進行渲染後，Yew 只會刪除第二個HTML \
             元素，而其他元素則保持不變，因為它可以使用鍵將它們關聯起來。",
        ],
        p![
            "如果你遇到了一個從一個元件切換到另一個元件的 bug/\"feature\"，但兩者都有一個 div \
             作為最高渲染元素。 Yew 在這些情況下會重複使用已渲染的 HTML div 作為最佳化。 \
             如果你需要該 div \
             被重新建立而不是被重複使用，那麼你可以添加不同的鍵，它們將不會被重複使用。"
        ],
        h2!["進一步閱讀"],
        ul![
            li![link!(
                "https://github.com/yewstack/yew/tree/master/examples/todomvc",
                "TodoMVC 範例",
            )],
            li![link!(
                "https://github.com/yewstack/yew/tree/master/examples/keyed_list",
                "帶有按鍵 (keys) 的清單範例",
            )],
            li![link!(
                "https://github.com/yewstack/yew/tree/master/examples/router",
                "路由範例",
            )],
        ],
    ])
}

crate::doc_page!("列表", "/zh-Hant/docs/concepts/html/lists", page_content());
