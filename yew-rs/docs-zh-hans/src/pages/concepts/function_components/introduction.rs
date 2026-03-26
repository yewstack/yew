pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p!["让我们重新回顾一下之前的标语："],
        p!["> Yew 的核心思想是将可重用的 UI 部分所需的所有内容集中在一个地方 - Rust 文件中。"],
        p!["我们将通过引入将定义应用程序的逻辑和呈现行为的概念来完善这个陈述：\"组件\"。"],
        h2!["什么是组件？"],
        p!["组件是 Yew 的构建块。"],
        p!["它们应当："],
        ul![
            li![
                "以 ",
                link!(
                    "/zh-Hans/docs/concepts/function-components/properties",
                    "Props",
                ),
                " 的形式接受参数",
            ],
            li!["可以拥有自己的状态"],
            li!["计算用户可见的 HTML 片段（DOM）"],
        ],
        h2!["Yew 组件的两种风味"],
        p!["您当前正在阅读有关函数组件的内容 - 这是在开始使用 Yew \
            时以及在编写简单的呈现逻辑时编写组件的推荐方式。"],
        p![
            "还有一种更高级但不太容易访问的编写组件的方式 - ",
            link!(
                "/zh-Hans/docs/advanced-topics/struct-components",
                "结构组件",
            ),
            "。它们允许非常详细的控制，尽管大多数情况下您不需要那么详细的控制。",
        ],
        h2!["创建函数组件"],
        p![
            "要创建一个函数组件，请将 ",
            code("#[component]"),
            " 属性添加到一个函数中。按照惯例，函数的名称采用 PascalCase，与 ",
            code("html!"),
            " 宏中的普通 html 元素形成对比。",
        ],
        code_block(
            "rust",
            r#"use yew::{component, html, Html};

#[component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

// 然后在其他地方，您可以在 `html!` 中使用组件
#[component]
fn App() -> Html {
    html! { <HelloWorld /> }
}"#,
        ),
        h2!["组件内部发生了什么"],
        p![
            "在渲染时，Yew 将构建这些组件的虚拟树。它将调用每个（函数）组件的 view 函数来计算 DOM \
             的虚拟版本（VDOM），您作为库用户将其视为 ",
            code("Html"),
            " 类型。对于上面的示例，这将如下所示：",
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
            "当需要更新时，Yew 将再次调用 view 函数，并将新的虚拟 DOM \
             与其之前的版本进行协调，并仅将新的/更改的/必要的部分传 播到实际的 \
             DOM。这就是我们所说的 ",
            bold!["渲染"],
            "。",
        ],
        admonition!(
            AdmonitionType::Note,
            None,
            p![
                "实际上，",
                code("Html"),
                " 只是 ",
                code("VNode"),
                " 的别名 - 一个虚拟节点。",
            ],
        ),
    ])
}

crate::doc_page!(
    "函数组件",
    "/zh-Hans/docs/concepts/function-components",
    page_content()
);
