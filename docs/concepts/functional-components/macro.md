---
title: #[functional_component]
description: The #[functional_component] attribute
---


The `#[functional_component]` attribute allows you automatically to generate the requirements for creating Yew functional components. Any Rust function can be annotated and used as a functional component. 

A function annotated with `#[functional_component]` may take one argument for props of `Properties` type's reference and must return `Html`. The name of component is passed as an attribute to the said attribute. Note that unlike struct component, the `Properties` type for functional components must also implement `PartialEq`.

### Example

<!--DOCUSAURUS_CODE_TABS-->
<!--With props-->
```rust
#[derive(Properties, Clone, PartialEq)]
pub struct RenderedAtProps {
    pub time: Date,
}

#[functional_component(RenderedAt)]
pub fn rendered_at(props: &RenderedAtProps) -> Html {
    html! {
        <p>
            <b>{ "Rendered at: " }</b>
            { String::from(props.time.to_string()) }
        </p>
    }
}
```

<!--Without props-->
```rust
#[functional_component(App)]
fn app() -> Html {
    let (counter, set_counter) = use_state(|| 0);

    let (counter_one, set_counter_one) = (counter.clone(), set_counter.clone());
    let inc_click = Callback::from(move |_| set_counter_one(*counter_one + 1));

    let (counter_two, set_counter_two) = (counter.clone(), set_counter);
    let dec_click = Callback::from(move |_| set_counter_two(*counter_two - 1));
    
    html! {<>
        <nav>
            <button onclick=inc_onclick>{ "Increment" }</button>
            <button onclick=dec_onclick>{ "Decrement" }</button>
        </nav>
        <p>
            <b>{ "Current value: " }</b>
            { counter }
        </p>
    </>}
}
```
<!--END_DOCUSAURUS_CODE_TABS-->

:::tip
It is possible to completely omit props argument is no props are to be passed.
:::

### `FunctionalProvider` trait

This attribute generates a struct which implements `FunctionalProvider` trait. A type alias `FunctionalComponent<CreatedStruct>` is then created. `FunctionalComponent` is a struct which implements `yew::Component` and handles the magic required for making functional components work. In most cases, you don't need to use the trait implementation, all you need is the aliased `type` which can be used as any other component.
