---
title: Classes
description: A handy macro to handle classes
---

## Classes

There are a number of convenient ways to specify classes for an element:

<!--DOCUSAURUS_CODE_TABS-->
<!--Literal-->

```rust
html! {
  <div class=html!("container")></div>
}
```

<!--Multiple-->

```rust
html! {
  <div class=html!("class-1", "class-2")></div>
}
```

<!--Optional-->

```rust
html! {
  <div class=html!(Some("class")) />
}
```

<!--Interpolated-->

```rust
html! {
  <div class=html!(format!("{}-container", size)) />
}
```

<!--Vector-->

```rust
html! {
  <div class=html!(vec!["class-1", "class-2"])></div>
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
