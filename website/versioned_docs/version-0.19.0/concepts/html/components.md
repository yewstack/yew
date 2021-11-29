---
title: "Components"
description: "Create complex layouts with component hierarchies"
---

## Basic

Any type that implements `Component` can be used in the `html!` macro:

```rust
use yew::{Component, Html, html, Context, Properties};

struct MyComponent;

impl Component for MyComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            { "This component has no properties!" }
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
struct Props {
    prop1: String,
    prop2: String,
}

struct MyComponentWithProps;

impl Component for MyComponentWithProps {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            {
                format!(
                    "prop1: {} and prop2: {}",
                    ctx.props().prop1,
                    ctx.props().prop2
                )
            }
        }
    }
}

let props = Props { 
    prop1: "Hello".to_owned(),
    prop2: "World".to_owned(), 
};


html!{
    <>
        // No properties
        <MyComponent />

        // With Properties
        <MyComponentWithProps prop1="lorem" prop2="ipsum" />

        // With the whole set of props provided at once
        <MyComponentWithProps ..props.clone() />

        // With Properties from a variable and specific values overridden
        <MyComponentWithProps prop2="lorem" ..props />
    </>
};
```

## Nested

Components can be passed children if they have a `children` field in their `Properties`.

```rust title="parent.rs"
use yew::{Children, Component, Context, html, Html, Properties};

#[derive(PartialEq, Properties)]
struct Props {
    id: String,
    children: Children,
}

struct Container;

impl Component for Container {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id={ctx.props().id.clone()}>
                { ctx.props().children.clone() }
            </div>
        }
    }
}

html! {
    <Container id="container">
        <h4>{ "Hi" }</h4>
        <div>{ "Hello" }</div>
    </Container>
};
```

The `html!` macro allows you to pass a base expression with the `..props` syntax instead of specifying each property individually,
similar to Rust's [Functional Update Syntax](https://doc.rust-lang.org/stable/reference/expressions/struct-expr.html#functional-update-syntax).
This base expression must occur after any individual props are passed.
When passing a base props expression with a `children` field, the children passed in the `html!` macro overwrite the ones already present in the props.

```rust
use yew::{Children, Component, Context, html, Html, props, Properties}; 

#[derive(PartialEq, Properties)]
struct Props {
    id: String,
    children: Children,
}

struct Container;

impl Component for Container {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id={ctx.props().id.clone()}>
                { ctx.props().children.clone() }
            </div>
        }
    }
}

let props = yew::props!(Container::Properties {
    id: "container-2",
    children: Children::default(),
});

html! {
    <Container ..props>
        // props.children will be overwritten with this
        <span>{ "I am a child, as you can see" }</span>
    </Container>
};
```

## Nested Children with Props

Nested component properties can be accessed and mutated if the containing component types its children. In the following example, the `List` component can wrap `ListItem` components. For a real world example of this pattern, check out the `yew-router` source code. For a more advanced example, check out the `nested-list` example in the main yew repository.

```rust
use std::rc::Rc;
use yew::{html, ChildrenWithProps, Component, Context, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct ListItemProps {
    value: String,
}

pub struct ListItem;

impl Component for ListItem {
    type Message = ();
    type Properties = ListItemProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <span>
                { ctx.props().value.clone() }
            </span>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: ChildrenWithProps<ListItem>,
}

pub struct List;
impl Component for List {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {{
            for ctx.props().children.iter().map(|mut item| {
                let mut props = Rc::make_mut(&mut item.props);
                props.value = format!("item-{}", props.value);
                item
            })
        }}
    }
}
html! {
    <List>
        <ListItem value="a" />
        <ListItem value="b" />
        <ListItem value="c" />
    </List>
};
```

## Relevant examples
- [Boids](https://github.com/yewstack/yew/tree/master/examples/boids)
- [Router](https://github.com/yewstack/yew/tree/master/examples/router)
- [Store](https://github.com/yewstack/yew/tree/master/examples/store)
