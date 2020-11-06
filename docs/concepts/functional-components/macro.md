---
title: #[functional_component]
description: The #[functional_component] attribute
---


The `#[functional_component]` attribute allows you automatically to generate the requirements for creating Yew functional components. Any Rust function can be annotated and used as a functional component. 

A function annotated with `#[functional_component]` may take one argument for props of `Properties` type's reference and must return `Html`. The name of component is passed as an attribute to the said attribute. Note that unlike struct component, the `Properties` type for functional components must also implement `PartialEq`.

### Example

```rust
#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

#[functional_component(Comp)]
fn comp(props: &Props) -> Html {
    html! {
        <p>
            { props.text }
        </p>
    }
}
``` 

:::tip
It is possible to completely omit props argument is no props are to be passed.
:::


