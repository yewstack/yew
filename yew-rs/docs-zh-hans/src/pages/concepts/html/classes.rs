pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["类"],
        p![code("Classes"), " 结构体可以用来处理 HTML 类。"],
        p![
            "将字符串推送到集合时，",
            code("Classes"),
            " 确保每个类都有一个元素，即使单个字符串可能包含多个类。",
        ],
        p![
            code("Classes"),
            " 也可以通过使用 ",
            code("Extend"),
            "（即 ",
            code("classes1.extend(classes2)"),
            "）或 ",
            code("push()"),
            "（即 ",
            code("classes1.push(classes2)"),
            "）来合并。任何实现 ",
            code("Into<Classes>"),
            " 的类型都可以推送到现有的 ",
            code("Classes"),
            " 上。",
        ],
        p![
            code("classes!"),
            " 是一个方便的宏，它创建一个单一的 ",
            code("Classes"),
            "。它的输入接受一个逗号分隔的表达式列表。唯一的要求是每个表达式都实现了 ",
            code("Into<Classes>"),
            "。",
        ],
        tabs![
            "Literal",
            tab![
                "Literal",
                "Literal",
                code_block(
                    "rust",
                    r#"use yew::{classes, html};

html! {
    <div class={classes!("container")}></div>
};"#,
                ),
            ],
            tab![
                "Multiple",
                "Multiple",
                code_block(
                    "rust",
                    r#"use yew::{classes, html};

html! {
  <div class={classes!("class-1", "class-2")}></div>
};"#,
                ),
            ],
            tab![
                "String",
                "String",
                code_block(
                    "rust",
                    r#"use yew::{classes, html};

let my_classes = String::from("class-1 class-2");

html! {
  <div class={classes!(my_classes)}></div>
};"#,
                ),
            ],
            tab![
                "Optional",
                "Optional",
                code_block(
                    "rust",
                    r#"use yew::{classes, html};

html! {
  <div class={classes!(Some("class"))} />
};"#,
                ),
            ],
            tab![
                "Vector",
                "Vector",
                code_block(
                    "rust",
                    r#"use yew::{classes, html};

html! {
  <div class={classes!(vec!["class-1", "class-2"])}></div>
};"#,
                ),
            ],
            tab![
                "Array",
                "Array",
                code_block(
                    "rust",
                    r#"use yew::{classes, html};

html! {
  <div class={classes!(["class-1", "class-2"])}></div>
};"#,
                ),
            ],
        ],
        h2!["接受类的组件"],
        code_block(
            "rust",
            r#"use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct Props {
    #[prop_or_default]
    class: Classes,
    fill: bool,
    children: Html,
}

#[component]
fn MyComponent(props: &Props) -> Html {
    let Props {
        class,
        fill,
        children,
    } = props;
    html! {
        <div
            class={classes!(
                "my-container-class",
                fill.then(|| Some("my-fill-class")),
                class.clone(),
            )}
        >
            { children.clone() }
        </div>
    }
}"#,
        ),
    ])
    .with_description("A handy macro to handle classes")
}

crate::doc_page!("类", "/zh-Hans/docs/concepts/html/classes", page_content());
