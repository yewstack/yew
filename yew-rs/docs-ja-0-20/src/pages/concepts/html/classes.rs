crate::doc_page!(
    "Classes",
    "/ja/docs/concepts/html/classes",
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
                 contain multiple classes."
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
            text("). In fact, anything that implements "),
            code("Into<Classes>"),
            text(" can be used to push new classes to the set."),
        ]),
        p(vec![
            text("The macro "),
            code("classes!"),
            text(" is a convenient macro that creates one single "),
            code("Classes"),
            text(
                ". Its input accepts a comma separated list of expressions. The only requirement \
                 is that every expression implements "
            ),
            code("Into<Classes>"),
            text("."),
        ]),
        p(vec![text("Literal:")]),
        code_block(
            "rust",
            r#"use yew::{classes, html};

html! {
<div class={classes!("container")}></div>
};"#
        ),
        p(vec![text("Multiple:")]),
        code_block(
            "rust",
            r#"use yew::{classes, html};

html! {
<div class={classes!("class-1", "class-2")}></div>
};"#
        ),
        p(vec![text("String:")]),
        code_block(
            "rust",
            r#"use yew::{classes, html};

let my_classes = String::from("class-1 class-2");

html! {
<div class={classes!(my_classes)}></div>
};"#
        ),
        p(vec![text("Optional:")]),
        code_block(
            "rust",
            r#"use yew::{classes, html};

html! {
<div class={classes!(Some("class"))} />
};"#
        ),
        p(vec![text("Vector:")]),
        code_block(
            "rust",
            r#"use yew::{classes, html};

html! {
<div class={classes!(vec!["class-1", "class-2"])}></div>
};"#
        ),
        p(vec![text("Array:")]),
        code_block(
            "rust",
            r#"use yew::{classes, html};

let my_classes = ["class-1", "class-2"];

html! {
<div class={classes!(my_classes.as_ref())}></div>
};"#
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
children: Children,
}

#[function_component(MyComponent)]
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
