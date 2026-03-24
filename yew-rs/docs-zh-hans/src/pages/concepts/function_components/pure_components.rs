pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            text("每个函数组件都是一个"),
            link!(
                "https://zh.wikipedia.org/wiki/%E7%BA%AF%E5%87%BD%E6%95%B0",
                text("纯"),
            ),
            text("函数，它接受一个属性对象并返回一个 "),
            code("Html"),
            text(" 对象。纯函数是指在给定相同输入时，总是返回相同输出的函数。"),
        ],
        p![
            text("这个例子是一个纯组件。对于给定的属性 "),
            code("is_loading"),
            text("，它总是返回相同的 "),
            code("Html"),
            text("，没有任何副作用。"),
        ],
        code_block(
            "rust",
            r#"use yew::{Properties, component, Html, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[component]
fn HelloWorld(props: &Props) -> Html {
    if props.is_loading {
        html! { "Loading" }
    } else {
        html! { "Hello world" }
    }
}"#,
        ),
        admonition![
            AdmonitionType::Note,
            None,
            p![
                text(
                    "如果您有一个内部纯组件，它不使用 hooks \
                     和其他组件机制，您通常可以将其编写为返回 ",
                ),
                code("Html"),
                text(" 的普通函数，从而避免 Yew 运行组件生命周期相关的一些开销。使用 "),
                link!(
                    "/zh-Hans/docs/concepts/html/literals-and-expressions#expressions",
                    text("表达式语法"),
                ),
                text(" 在 "),
                code("html!"),
                text(" 中渲染它们。"),
            ],
        ],
        h2![text("非纯组件")],
        p![
            text(
                "您可能想知道，如果组件不使用任何全局变量，那么它是否可以是不\"纯\"的，\
                 因为它只是在每次渲染时调用的固定函数。 这就是下一个主题 - ",
            ),
            link!(
                "/zh-Hans/docs/concepts/function-components/hooks",
                text("hooks"),
            ),
            text(" 的用武之地。"),
        ],
    ])
}

crate::doc_page!(
    "纯组件",
    "/zh-Hans/docs/concepts/function-components/pure-components",
    page_content()
);
