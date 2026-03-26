crate::doc_page!(
    "Classes",
    "/docs/concepts/html/classes",
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
        p!["Literal:"],
        code_block(
            "rust",
            r#"use yew::{classes, html};

html! {
<div class={classes!("container")}></div>
};"#
        ),
        p!["Multiple:"],
        code_block(
            "rust",
            r#"use yew::{classes, html};

html! {
<div class={classes!("class-1", "class-2")}></div>
};"#
        ),
        p!["String:"],
        code_block(
            "rust",
            r#"use yew::{classes, html};

let my_classes = String::from("class-1 class-2");

html! {
<div class={classes!(my_classes)}></div>
};"#
        ),
        p!["Optional:"],
        code_block(
            "rust",
            r#"use yew::{classes, html};

html! {
<div class={classes!(Some("class"))} />
};"#
        ),
        p!["Vector:"],
        code_block(
            "rust",
            r#"use yew::{classes, html};

html! {
<div class={classes!(vec!["class-1", "class-2"])}></div>
};"#
        ),
        p!["Array:"],
        code_block(
            "rust",
            r#"use yew::{classes, html};

let my_classes = ["class-1", "class-2"];

html! {
<div class={classes!(my_classes.as_ref())}></div>
};"#
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
}"#
        ),
    ])
);
