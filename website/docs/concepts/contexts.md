---
title: "Contexts"
sidebar_label: Contexts
description: "Using contexts to pass data within application"
---

Generally data is passed down the component tree using props but that becomes tedious for values such as 
user preferences, authentication information etc. Consider the following example which passes down the 
theme using props:
```rust
// root
let theme = // ...
html! {
    <Navbar theme=theme />
}

// Navbar component
html! {
    <div>
        <Title theme=theme>{ "App title" }<Title>
        <NavButton theme=theme>{ "Somewhere" }</NavButton>
    </div>
}
```

Passing down data like this isn't ideal for something like a theme which needs to be available everywhere. 
This is where contexts come in.

Contexts provide a way to share data between components without passing them down explicitly as props.
They make data available to all components in the tree.

## Using Contexts

In order to use contexts, we need a struct which defines what data is to be passed.
For the above use-case, consider the following struct:
```rust
#[derive(Clone, Debug, PartialEq)]
struct Theme {
    foreground: String,
    background: String,
}
```

A context provider is required to consume the context. `ContextProvider<T>`, where `T` is the context struct is used as the provider.
`T` must implement `Clone` and `PartialEq`. `ContextProvider` is the component whose children will have the context available to them.
The children are re-rendered when the context changes.

### Consuming context

#### Struct components

The `ComponentLink::context` method is used to consume contexts in struct components.

##### Example

```rust
struct ContextDemo {
    link: ComponentLink<Self> 
}

impl Component for ContextDemo {
    /// ...
    fn view(&self) -> Html {
        let theme = self.link.context::<Theme>();
        html! {
            <button style=format!("background: {}; color: {};", theme.background, theme.foreground)>
                { "Click me!" }
            </button>
        }
    }
}
```

#### Function components

`use_context` hook is used to consume contexts in function components. 
See [docs for use_context](function-components/pre-defined-hooks.md#use_context) to learn more.
