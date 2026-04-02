crate::doc_page!(
    "Classes",
    "/zh-Hant/docs/concepts/html/classes",
    Content::new(vec![
        h2!["Classes"],
        p![
            "The struct ",
            code("Classes"),
            " can be used to deal with HTML classes.",
        ],
        p![
            "When pushing a string to the set, ",
            code("Classes"),
            " ensures that there is one element for every class even if a single string might \
             contain multiple classes.",
        ],
        p![
            code("Classes"),
            " can also be merged by using ",
            code("Extend"),
            " (i.e. ",
            code("classes1.extend(classes2)"),
            ") or ",
            code("push()"),
            " (i.e. ",
            code("classes1.push(classes2)"),
            "). In fact, anything that implements ",
            code("Into<Classes>"),
            " can be used to push new classes to the set.",
        ],
        p![
            "The macro ",
            code("classes!"),
            " is a convenient macro that creates one single ",
            code("Classes"),
            ". Its input accepts a comma separated list of expressions. The only requirement is \
             that every expression implements ",
            code("Into<Classes>"),
            ".",
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
        h2!["Components that accept classes"],
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
    .with_description("A handy macro to handle classes")
);
