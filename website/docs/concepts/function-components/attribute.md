---
title: "#[function_component]"
description: "The #[function_component] attribute"
---

`#[function_component(_)]` turns a normal Rust function into a function component.
Functions with the attribute have to return `Html` and may take a single parameter for the type of props the component should accept.
The parameter type needs to be a reference to a type which implements `Properties` and `PartialEq` (ex. `props: &MyProps`).
If the function doesn't have any parameters the resulting component doesn't accept any props.

The attribute doesn't replace your original function with a component. You need to provide a name as an input to the attribute which will be the identifier of the component.
Assuming you have a function called `chat_container` and you add the attribute `#[function_component(ChatContainer)]` you can use the component like this:

```rust
html! { <ChatContainer /> }
```

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

    let onclick = {
        let counter = Rc::clone(&counter);
        Callback::from(move |_| set_counter(*counter + 1))
    };

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

## Generic function components

The `#[function_component(_)]` attribute also works with generic functions for creating generic components.

```rust
#[derive(Properties, Clone, PartialEq)]
pub struct Props<T>
    where T: Clone + PartialEq
{
    data: T,
}

#[function_component(MyGenericComponent)]
pub fn my_generic_component<T>(props: &Props<T>) -> Html
    where T: Clone + PartialEq + Display
{
    html! {
        <p>
            { props.data }
        </p>
    }
}

// used like this
html! {
    <MyGenericComponent<i32> data=123 />
}
// or
html! {
    <MyGenericComponent<Foo> data=foo />
}
```
