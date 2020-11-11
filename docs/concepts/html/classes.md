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
to the set. This includes: `&'static str`, `String`, `Cow<'static, str>`,
but also: `Vec<impl Into<Classes>>`, `&[impl Into<Classes>]`,
`Option<impl Into<Classes>>` and `&Option<impl Into<Classes>>`. For example
`Option<String>` can be converted to `Classes` and get an empty set if the
option is None (or the string is empty), but a set of one or more elements if
the string contains one or more classes.

The macro `classes!` is a convenient macro that creates one single `Classes`.
Its input accepts a comma separated list of expressions. The only requirement
is that every expression implements `Into<Classes>`.

<!--DOCUSAURUS_CODE_TABS-->
<!--Strings & literals-->

```rust
use boolinator::Boolinator;

#[derive(Clone, Properties)]
struct Props {
    size: u32,
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
            size,
            children,
        } = &self.props;
        html! {
            <div
                class=classes!(
                    "my-container-class",
                    format!("{}-container", size)
                )
            >
                { children.clone() }
            </div>
        }
    }
}
```

<!--Multiple sets of classes-->

```rust
use boolinator::Boolinator;

#[derive(Clone, Properties)]
struct Props {
    // Classes defaults to an empty set
    #[prop_or_default]
    class: Classes,
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
            children,
        } = &self.props;
        html! {
            <div
                class=classes!(
                    "my-container-class",
                    class.clone(),
                )
            >
                { children.clone() }
            </div>
        }
    }
}
```

<!--Optional-->

```rust
use boolinator::Boolinator;

#[derive(Clone, Properties)]
struct Props {
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
            fill,
            children,
        } = &self.props;
        html! {
            <div
                class=classes!(
                    "my-container-class",
                    fill.as_some("my-fill-class"),
                )
            >
                { children.clone() }
            </div>
        }
    }
}
```

<!--Generating Classes from a Vec-->

```rust
use boolinator::Boolinator;

#[derive(Clone, Properties)]
struct Props {
    #[prop_or_default]
    tags: Vec<String>,
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
            tags,
            children,
        } = &self.props;
        let all_classes = tags
            .iter()
            .map(|tag| format!("tag-{}", tag))
            .collect::<Classes>();
        html! {
            <div
                class=classes!(
                    "my-container-class",
                    all_classes,
                )
            >
                { children.clone() }
            </div>
        }
    }
}
```

<!--END_DOCUSAURUS_CODE_TABS-->

The example makes use of the [boolinator](https://crates.io/crates/boolinator)
crate to conditionally add the "my-fill-class" class based on the `fill`
boolean attribute.
