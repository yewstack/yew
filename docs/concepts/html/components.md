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
    <Container id="container">
        <h4>{ "Hi" }</h4>
        <div>{ "Hello" }</div>
    </Container>
}
```

When using the `with props` syntax, the children passed in the `html!` macro overwrite the ones already present in the props.

```rust
let props = yew::props!(Container::Properties {
    id: "container-2",
    children: Children::default(),
});
html! {
    <Container with props>
        // props.children will be overwritten with this
        <span>{ "I am a child, as you can see" }</span>
    </Container>
}
```

Here's the implementation of `Container`:

```rust
#[derive(Properties, Clone)]
pub struct Props {
    pub id: String,
    pub children: Children,
}

pub struct Container(Props);
impl Component for Container {
    type Properties = Props;

    // ...

    fn view(&self) -> Html {
       html! {
           <div id=&self.0.id>
               { self.0.children.clone() }
           </div>
       }
    }
}
```

## Nested Children with Props

Nested component properties can be accessed and mutated if the containing component types its children. In the following example, the `List` component can wrap `ListItem` components. For a real world example of this pattern, check out the `yew-router` source code. For a more advanced example, check out the `nested-list` example in the main yew repository.

```rust
html! {
    <List>
        <ListItem value="a" />
        <ListItem value="b" />
        <ListItem value="c" />
    </List>
}
```

```rust
#[derive(Properties, Clone)]
pub struct Props {
    pub children: ChildrenWithProps<ListItem>,
}

pub struct List(Props);
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

## Transformers

Whenever you set a prop its value goes through a transformation step first.
If the value already has the correct type, this step doesn't do anything.
However, transformers can be useful to reduce code repetition.

The following is a list of transformers you should know about:

- `&T` -> `T`

  Clones the reference to get an owned value.

- `&str` -> `String`

  Allows you to use string literals without adding `.to_owned()` at the end.

- `T` -> `Option<T>`

  Wraps the value in `Some`.

```rust
struct Props {
    unique_id: Option<usize>,
    text: String,
}

struct Model;
impl Component for Model {
    type Properties = Props;

    // ...
}

// transformers allow you to write this:
html! { <Model unique_id=5 text="literals are fun" /> };
// instead of:
html! { <Model unique_id=Some(5) text="literals are fun".to_owned() /> };
```

## Relevant examples
- [Boids](https://github.com/yewstack/yew/tree/master/examples/boids)
- [Router](https://github.com/yewstack/yew/tree/master/examples/router)
- [Store](https://github.com/yewstack/yew/tree/master/examples/store)
