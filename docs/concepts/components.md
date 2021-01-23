---
title: Introduction
description: Components and their lifecycle hooks
---

## What are Components?

Components are the building blocks of Yew. They manage their own state and can render themselves to the DOM. Components are created by implementing the `Component` trait for a type. The `Component`
trait has a number of methods which need to be implemented; Yew will call these at different stages
in the lifecycle of a component.

## Lifecycle

:::important contribute
`Contribute to our docs:` [Add a diagram of the component lifecycle](https://github.com/yewstack/docs/issues/22)
:::

## Lifecycle Methods

### Create

When a component is created, it receives properties from its parent component as well as a `ComponentLink`. The properties can be used to initialize the component's state and the "link" can be used to register callbacks or send messages to the component.

It is common to store the props (data which can be passed from parent to child components) and the
`ComponentLink` in your component struct, like so:

```rust
pub struct MyComponent {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for MyComponent {
    type Properties = Props;
    // ...

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MyComponent { props, link }
    }

    // ...
}
```

### View

The `view` method allows you to describe how a component should be rendered to the DOM. Writing
HTML-like code using Rust functions can become quite messy, so Yew provides a macro called `html!`
for declaring HTML and SVG nodes (as well as attaching attributes and event listeners to them) and a
convenient way to render child components. The macro is somewhat similar to React's JSX (the
differences in programming language aside).

```rust
impl Component for MyComponent {
    // ...

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::Click);
        html! {
            <button onclick=onclick>{ self.props.button_text }</button>
        }
    }
}
```

For usage details, check out [the `html!` guide](html.md).

### Rendered

The `rendered` component lifecycle method is called once `view` has been called and Yew has rendered
the results to the DOM, but before the browser refreshes the page. This method is useful when you
want to perform actions that can only be completed after the component has rendered elements. There
is also a parameter called `first_render` which can be used to determine whether this function is
being called on the first render, or instead a subsequent one.

```rust
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct MyComponent {
    node_ref: NodeRef,
}

impl Component for MyComponent {
    // ...

    fn view(&self) -> Html {
        html! {
            <input ref=self.node_ref.clone() type="text" />
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
                input.focus();
            }
        }
    }
}
```

:::tip note
Note that this lifecycle method does not require an implementation and will do nothing by default.
:::

### Update

Communication with components happens primarily through messages which are handled by the
`update` lifecycle method. This allows the component to update itself
based on what the message was, and determine if it needs to re-render itself. Messages can be sent
by event listeners, child components, Agents, Services, or Futures.

Here's an example of what an implementation of `update` could look like:

```rust
pub enum Msg {
    SetInputEnabled(bool)
}

impl Component for MyComponent {
    type Message = Msg;

    // ...

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
       match msg {
           Msg::SetInputEnabled(enabled) => {
               if self.input_enabled != enabled {
                   self.input_enabled = enabled;
                   true // Re-render
               } else {
                   false
               }
           }
       }
    }
}
```

### Change

Components may be re-rendered by their parents. When this happens, they could receive new properties
and need to re-render. This design facilitates parent to child component communication by just
changing the values of a property.

A typical implementation would look something like:

```rust
impl Component for MyComponent {
    // ...

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }
}
```

### Destroy

After Components are unmounted from the DOM, Yew calls the `destroy` lifecycle method; this is
necessary if you need to undertake operations to clean up after earlier actions of a component
before it is destroyed. This method is optional and does nothing by default.

## Associated Types

The `Component` trait has two associated types: `Message` and `Properties`.

```rust
impl Component for MyComponent {
    type Message = Msg;
    type Properties = Props;

    // ...
}
```

The `Message` type is used to send messages to a component after an event has taken place; for
example you might want to undertake some action when a user clicks a button or scrolls down the
page. Because components tend to have to respond to more than one event, the `Message` type will
normally be an enum, where each variant is an event to be handled.

When organizing your codebase, it is sensible to include the definition of the `Message` type in the
same module in which your component is defined. You may find it helpful to adopt a consistent naming
convention for message types. One option (though not the only one) is to name the types
`ComponentNameMsg`, e.g. if your component was called `Homepage` then you might call the type
`HomepageMsg`.

```rust
enum Msg {
    Click,
    FormInput(String)
}
```

`Properties` represents the information passed to a component from its parent. This type must implements the `Properties` trait \(usually by deriving it\) and can specify whether certain properties are required or optional. This type is used when creating and updating a component. It is common practice to create a struct called `Props` in your component's module and use that as the component's `Properties` type. It is common to shorten "properties" to "props". Since props are handed down from parent components, the root component of your application typically has a `Properties` type of `()`. If you wish to specify properties for your root component, use the `App::mount_with_props` method.
