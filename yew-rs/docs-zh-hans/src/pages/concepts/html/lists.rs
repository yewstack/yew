pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["迭代器"],
        p!["从迭代器构建 HTML 有 3 种方法："],
        tabs![
            "`for` 循环",
            tab![
                "`for` 循环",
                "`for` 循环",
                p!["主要方法是使用 for 循环，与 Rust 中已有的 for 循环相同，但有 2 个关键区别："],
                ol![
                    li![
                        "与标准 for 循环不能返回任何内容不同，",
                        code("html!"),
                        " 中的 for 循环会被转换为节点列表；",
                    ],
                    li![
                        "发散表达式，即 ",
                        code("break"),
                        "、",
                        code("continue"),
                        " 在 ",
                        code("html!"),
                        " 中的 for 循环体内是不允许的。",
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
                "`for` 块",
                "`for` 块",
                p![
                    "另一种方法是使用 ",
                    code("for"),
                    " 关键字，这不是原生的 Rust 语法，而是由 HTML \
                     宏用于输出显示迭代器所需的代码。当迭代器已经计算好，只需要将其传递给宏时，\
                     这种方法比第一种更好。",
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
                    "最后一种方法是在迭代器的最终转换上调用 ",
                    code("collect::<Html>()"),
                    "，它返回一个 Yew 可以显示的列表。",
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
        h2!["键 (Key) 列表"],
        p![
            "键 (Key) 列表是一个优化的列表，其中",
            bold!["所有"],
            "子元素都有键。 ",
            code("key"),
            " 是 Yew 提供的一个特殊属性，它为 HTML 元素或组件提供一个唯一标识符，用于 Yew \
             内部的优化。",
        ],
        admonition![
            AdmonitionType::Caution,
            None,
            p![
                "Key 只需要在每个列表中是唯一的，与 HTML ",
                code("id"),
                " 的全局唯一性相反。它不应该依赖于列表的顺序。",
            ],
        ],
        p!["始终建议为列表添加键 (key)。"],
        p![
            "可以通过将唯一的 ",
            code("String"),
            "、",
            code("str"),
            " 或整数传递给特殊的 ",
            code("key"),
            " 属性来添加键：",
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
};"#,
        ),
        h3!["性能优化"],
        p![
            "我们有一个",
            link!(
                "https://github.com/yewstack/yew/tree/master/examples/keyed_list",
                "带有键 (keys) 的列表示例",
            ),
            "可以让你测试性能上的改进，这里是一个简单的测试流程：",
        ],
        ol![
            li![
                "进入",
                link!("https://examples.yew.rs/keyed_list", "在线演示"),
            ],
            li!["添加 500 个元素"],
            li!["禁用键"],
            li!["反转列表"],
            li!["查看 \"最后一次渲染花费了 Xms\"（在撰写本文时，大约为 60ms）"],
            li!["启用键"],
            li!["再次反转列表"],
            li!["查看 \"最后一次渲染花费了 Xms\"（在撰写本文时，大约为 30ms）"],
        ],
        p!["截至撰写本文时，对于 500 个组件，速度提高了 2 倍。"],
        h3!["原理解释"],
        p![
            "通常，当你迭代时，只需要在每个列表项上添加一个键，数据的顺序可能会发生变化。 \
             在重新渲染列表时，它用于加速协调过程。"
        ],
        p![
            "如果没有键，假设你迭代 ",
            code("[\"bob\", \"sam\", \"rob\"]"),
            "，最终得到的 HTML 如下：",
        ],
        code_block(
            "html",
            r#"<div id="bob">My name is Bob</div>
<div id="sam">My name is Sam</div>
<div id="rob">My name is rob</div>"#,
        ),
        p![
            "然后在下一次渲染时，如果你的列表更改为 ",
            code("[\"bob\", \"rob\"]"),
            "，Yew 可以删除 id=\"rob\" 的元素，并将 id=\"sam\" 更新为 id=\"rob\"。",
        ],
        p![
            "如果你为每个元素添加了一个键，初始 HTML 将保持不变，但在使用修改后的列表 ",
            code("[\"bob\", \"rob\"]"),
            " 进行渲染后，Yew 只会删除第二个 HTML \
             元素，而其他元素则保持不变，因为它可以使用键将它们关联起来。",
        ],
        p![
            "如果你遇到了一个从一个组件切换到另一个组件的 bug/\"feature\"，但两者都有一个 div \
             作为最高渲染元素。 Yew 在这些情况下会重用已渲染的 HTML div 作为优化。 如果你需要该 \
             div 被重新创建而不是被重用，那么你可以添加不同的键，它们将不会被重用。"
        ],
        h2!["进一步阅读"],
        ul![
            li![link!(
                "https://github.com/yewstack/yew/tree/master/examples/todomvc",
                "TodoMVC 示例",
            )],
            li![link!(
                "https://github.com/yewstack/yew/tree/master/examples/keyed_list",
                "带有键 (keys) 的列表示例",
            )],
            li![link!(
                "https://github.com/yewstack/yew/tree/master/examples/router",
                "路由示例",
            )],
        ],
    ])
}

crate::doc_page!("列表", "/zh-Hans/docs/concepts/html/lists", page_content());
