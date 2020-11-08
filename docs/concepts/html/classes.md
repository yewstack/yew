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

## Using [boolinator](https://crates.io/crates/boolinator)

```rust
struct MyComponent {
    props: MyComponentProps,
}

#[derive(Clone, Properties)]
struct MyComponentProps {
    fill: bool,
    class: Option<String>,
    children: html::Children,
}

impl Component for MyComponent {
    type Message = ();
    type Properties = MyComponentProps;

    fn view(&self) -> Html {
        html! {
            <div
                class=classes!(
                    "my-container-class",
                    self.props.fill.as_some("my-fill-class"),
                    self.props.class.clone(),
                )
            >
                { self.props.children.clone() }
            </div>
        }
    }

    // ...
}
```
