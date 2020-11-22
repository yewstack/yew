---
title: Classes
description: A handy macro to handle classes
---

## Classes

The struct `Classes` can be used to deal with HTML classes.

When pushing a string to the set, `Classes` ensures that there is one element
for every class even if a single string might contain multiple classes.

`Classes` can also be merged by using `Extend` (i.e.
`classes1.extend(classes2)`) or `push()` (i.e. `classes1.push(classes2)`). In
fact, anything that implements `Into<Classes>` can be used to push new classes
to the set.

The macro `classes!` is a convenient macro that creates one single `Classes`.
Its input accepts a comma separated list of expressions. The only requirement
is that every expression implements `Into<Classes>`.

<!--DOCUSAURUS_CODE_TABS-->
<!--Literal-->

```rust
html! {
  <div class=classes!("container")></div>
}
```

<!--Multiple-->

```rust
html! {
  <div class=classes!("class-1", "class-2")></div>
}
```

<!--String-->

```rust
let my_classes = String::from("class-1 class-2");

html! {
  <div class=classes!(my_classes)></div>
}
```

<!--Optional-->

```rust
html! {
  <div class=classes!(Some("class")) />
}
```

<!--Vector-->

```rust
html! {
  <div class=classes!(vec!["class-1", "class-2"])></div>
}
```

<!--Array-->

```rust
let my_classes = ["class-1", "class-2"];

html! {
  <div class=classes!(&my_classes)></div>
}
```

<!--END_DOCUSAURUS_CODE_TABS-->

## Components that accept classes

```rust
use boolinator::Boolinator;

#[derive(Clone, Properties)]
struct Props {
    #[prop_or_default]
    class: Classes,
    fill: bool,
    children: Children,
}

struct MyComponent {
    props: Props,
}

impl Component for MyComponent {
    type Properties = Props;

    // ...

    fn view(&self) -> Html {
        let Props {
            class,
            fill,
            children,
        } = &self.props;
        html! {
            <div
                class=classes!(
                    "my-container-class",
                    fill.as_some("my-fill-class"),
                    class.clone(),
                )
            >
                { children.clone() }
            </div>
        }
    }
}
```

The example makes use of the [boolinator](https://crates.io/crates/boolinator)
crate to conditionally add the "my-fill-class" class based on the `fill`
boolean attribute.
