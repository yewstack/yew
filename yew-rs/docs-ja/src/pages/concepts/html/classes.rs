pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("クラス")],
        p![
            code("Classes"),
            text(" 構造体は、HTML クラスを処理するために使用できます。"),
        ],
        p![
            text("文字列をコレクションにプッシュすると、"),
            code("Classes"),
            text(
                " は各クラスが一つの要素を持つことを保証します。\
                 単一の文字列が複数のクラスを含む場合でも同様です。",
            ),
        ],
        p![
            code("Classes"),
            text(" は、"),
            code("Extend"),
            text("（例："),
            code("classes1.extend(classes2)"),
            text("）や "),
            code("push()"),
            text("（例："),
            code("classes1.push(classes2)"),
            text("）を使用してマージすることもできます。"),
            code("Into<Classes>"),
            text(" を実装している任意の型を既存の "),
            code("Classes"),
            text(" にプッシュすることができます。"),
        ],
        p![
            code("classes!"),
            text(" は、単一の "),
            code("Classes"),
            text(
                " を作成するための便利なマクロです。\
                 その入力はカンマで区切られた式のリストを受け入れます。唯一の要件は、各式が ",
            ),
            code("Into<Classes>"),
            text(" を実装していることです。"),
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
        h2![text("クラスを受け入れるコンポーネント")],
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
}

crate::doc_page!("クラス", "/ja/docs/concepts/html/classes", page_content());
