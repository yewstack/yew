crate::doc_page!(
    "Classes",
    "/zh-Hant/docs/concepts/html/classes",
    Content::new(vec![
        h2![text("Classes")],
        p![
            text("The struct "),
            code("Classes"),
            text(" can be used to deal with HTML classes."),
        ],
        p![
            text("When pushing a string to the set, "),
            code("Classes"),
            text(
                " ensures that there is one element for every class even if a single string might \
                 contain multiple classes.",
            ),
        ],
        p![
            code("Classes"),
            text(" can also be merged by using "),
            code("Extend"),
            text(" (i.e. "),
            code("classes1.extend(classes2)"),
            text(") or "),
            code("push()"),
            text(" (i.e. "),
            code("classes1.push(classes2)"),
            text("). In fact, anything that implements "),
            code("Into<Classes>"),
            text(" can be used to push new classes to the set."),
        ],
        p![
            text("The macro "),
            code("classes!"),
            text(" is a convenient macro that creates one single "),
            code("Classes"),
            text(
                ". Its input accepts a comma separated list of expressions. The only requirement \
                 is that every expression implements ",
            ),
            code("Into<Classes>"),
            text("."),
        ],
        tabs!(
            "Literal",
            tab!(
                "Literal",
                "Literal",
                code_block(
                    "rust",
                    "use yew::{classes, html};\n\nhtml! {\n    <div \
                     class={classes!(\"container\")}></div>\n};",
                ),
            ),
            tab!(
                "Multiple",
                "Multiple",
                code_block(
                    "rust",
                    "use yew::{classes, html};\n\nhtml! {\n  <div class={classes!(\"class-1\", \
                     \"class-2\")}></div>\n};",
                ),
            ),
            tab!(
                "String",
                "String",
                code_block(
                    "rust",
                    "use yew::{classes, html};\n\nlet my_classes = String::from(\"class-1 \
                     class-2\");\n\nhtml! {\n  <div class={classes!(my_classes)}></div>\n};",
                ),
            ),
            tab!(
                "Optional",
                "Optional",
                code_block(
                    "rust",
                    "use yew::{classes, html};\n\nhtml! {\n  <div \
                     class={classes!(Some(\"class\"))} />\n};",
                ),
            ),
            tab!(
                "Vector",
                "Vector",
                code_block(
                    "rust",
                    "use yew::{classes, html};\n\nhtml! {\n  <div \
                     class={classes!(vec![\"class-1\", \"class-2\"])}></div>\n};",
                ),
            ),
            tab!(
                "Array",
                "Array",
                code_block(
                    "rust",
                    "use yew::{classes, html};\n\nlet my_classes = [\"class-1\", \
                     \"class-2\"];\n\nhtml! {\n  <div \
                     class={classes!(my_classes.as_ref())}></div>\n};",
                ),
            ),
        ),
        h2![text("Components that accept classes")],
        code_block(
            "rust",
            r#"use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct Props {
    #[prop_or_default]
    class: Classes,
    fill: bool,
    children: Children,
}

#[function_component]
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
);
