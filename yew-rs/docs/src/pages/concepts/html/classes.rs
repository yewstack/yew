pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("Classes")]),
        p(vec![
            text("The struct "),
            code("Classes"),
            text(" can be used to deal with HTML classes."),
        ]),
        p(vec![
            text("When pushing a string to the set, "),
            code("Classes"),
            text(
                " ensures that there is one element for every class even if a single string might \
                 contain multiple classes.",
            ),
        ]),
        p(vec![
            code("Classes"),
            text(" can also be merged by using "),
            code("Extend"),
            text(" (i.e. "),
            code("classes1.extend(classes2)"),
            text(") or "),
            code("push()"),
            text(" (i.e. "),
            code("classes1.push(classes2)"),
            text("). Any type that implements "),
            code("Into<Classes>"),
            text(" can be pushed onto an existing "),
            code("Classes"),
            text("."),
        ]),
        p(vec![
            text("The macro "),
            code("classes!"),
            text(" is a convenient macro that creates one single "),
            code("Classes"),
            text(
                ". Its input accepts a comma-separated list of expressions. The only requirement \
                 is that every expression implements ",
            ),
            code("Into<Classes>"),
            text("."),
        ]),
        tabs(
            "Literal",
            vec![
                tab(
                    "Literal",
                    "Literal",
                    vec![code_block(
                        "rust",
                        r#"use yew::{classes, html};

html! {
    <div class={classes!("container")}></div>
};"#,
                    )],
                ),
                tab(
                    "Multiple",
                    "Multiple",
                    vec![code_block(
                        "rust",
                        r#"use yew::{classes, html};

html! {
  <div class={classes!("class-1", "class-2")}></div>
};"#,
                    )],
                ),
                tab(
                    "String",
                    "String",
                    vec![code_block(
                        "rust",
                        r#"use yew::{classes, html};

let my_classes = String::from("class-1 class-2");

html! {
  <div class={classes!(my_classes)}></div>
};"#,
                    )],
                ),
                tab(
                    "Optional",
                    "Optional",
                    vec![code_block(
                        "rust",
                        r#"use yew::{classes, html};

html! {
  <div class={classes!(Some("class"))} />
};"#,
                    )],
                ),
                tab(
                    "Vector",
                    "Vector",
                    vec![code_block(
                        "rust",
                        r#"use yew::{classes, html};

html! {
  <div class={classes!(vec!["class-1", "class-2"])}></div>
};"#,
                    )],
                ),
                tab(
                    "Array",
                    "Array",
                    vec![code_block(
                        "rust",
                        r#"use yew::{classes, html};

html! {
  <div class={classes!(["class-1", "class-2"])}></div>
};"#,
                    )],
                ),
            ],
        ),
        h2(vec![text("Components that accept classes")]),
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

crate::doc_page!("Classes", "/docs/concepts/html/classes", page_content());
