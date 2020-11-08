---
title: #[function_component]
description: The #[function_component] attribute
---


The `#[function_component]` attribute allows you automatically to generate the requirements for creating Yew function components. Any Rust function can be annotated and used as a function component. 

A function annotated with `#[function_component]` may take one argument for props of `Properties` type's reference and must return `Html`. The name of component is passed as an attribute to the said attribute. Note that unlike struct component, the `Properties` type for function components must also implement `PartialEq`.

## Example

<!--DOCUSAURUS_CODE_TABS-->
<!--With props-->
```rust
#[derive(Properties, Clone, PartialEq)]
pub struct RenderedAtProps {
    pub time: String,
}

#[function_component(RenderedAt)]
pub fn rendered_at(props: &RenderedAtProps) -> Html {
    html! {
        <p>
            <b>{ "Rendered at: " }</b>
            { &props.time }
        </p>
    }
}
```

<!--Without props-->
```rust
#[function_component(App)]
fn app() -> Html {
    let (counter, set_counter) = use_state(|| 0);
    let onclick = Callback::from(move |_| set_counter(*counter + 1));
    
    html! {
        <div>
            <button onclick=onclick>{ "Increment value" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { counter }
            </p>
        </div>
    }
}
```
<!--END_DOCUSAURUS_CODE_TABS-->
