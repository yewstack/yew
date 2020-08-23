---
title: Components
description: Create complex layouts with component hierarchies
---
## Basic

Any type that implements `Component` can be used in the `html!` macro:

```rust
html!{
    <>
        // No properties
        <MyComponent />

        // With Properties
        <MyComponent prop1="lorem" prop2="ipsum" />

        // With the whole set of props provided at once
        <MyComponent with props />
    </>
}
```

## Nested

Components can be passed children if they have a `children` field in their `Properties`.

```rust title="parent.rs"
html! {
    <Container>
        <h4>{ "Hi" }</h4>
        <div>{ "Hello" }</div>
    </Container>
}
```

```rust title="container.rs"
pub struct Container(Props);

#[derive(Properties, Clone)]
pub struct Props {
    pub children: Children,
}

impl Component for Container {
    type Properties = Props;

    // ...

    fn view(&self) -> Html {
       html! {
           <div id="container">
               { self.0.children.clone() }
           </div>
       }
    }
}
```

:::note
Types for which you derive `Properties` must also implement `Clone`. This can be done by either using `#[derive(Properties, Clone)]` or manually implementing `Clone` for your type.
:::

## Nested Children with Props

Nested component properties can be accessed and mutated if the containing component types its children. In the following example, the `List` component can wrap `ListItem` components. For a real world example of this pattern, check out the `yew-router` source code. For a more advanced example, check out the `nested-list` example in the main yew repository.

```rust title="parent.rs"
html! {
    <List>
        <ListItem value="a" />
        <ListItem value="b" />
        <ListItem value="c" />
    </List>
}
```

```rust title="list.rs"
pub struct List(Props);

#[derive(Properties, Clone)]
pub struct Props {
    pub children: ChildrenWithProps<ListItem>,
}

impl Component for List {
    type Properties = Props;

    // ...

    fn view(&self) -> Html {
        html!{{
            for self.0.children.iter().map(|mut item| {
                item.props.value = format!("item-{}", item.props.value);
                item
            })
        }}
    }
}
```

