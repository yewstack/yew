crate::doc_page!(
    "類別",
    "/zh-Hant/docs/concepts/html/classes",
    Content::new(vec![
        h2!["類別"],
        p![code("Classes"), " 結構體可以用來處理 HTML 類別。",],
        p![
            "將字串推送到集合時，",
            code("Classes"),
            " 確保每個類別都有一個元素，即使單一字串可能包含多個類別。",
        ],
        p![
            code("Classes"),
            " 也可以透過使用 ",
            code("Extend"),
            "（即 ",
            code("classes1.extend(classes2)"),
            "）或 ",
            code("push()"),
            "（即 ",
            code("classes1.push(classes2)"),
            "）來合併。任何實作 ",
            code("Into<Classes>"),
            " 的類型都可以推送到現有的 ",
            code("Classes"),
            " 上。",
        ],
        p![
            code("classes!"),
            " 是一個方便的巨集，它建立一個單一的 ",
            code("Classes"),
            "。它的輸入接受一個逗號分隔的表達式清單。唯一的要求是每個表達式都實作了 ",
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
        h2!["接受類別的元件"],
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
);
