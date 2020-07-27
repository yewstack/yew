---
id: components-intro
title: Introduction
description: Components and their lifecycle hooks
---
## What are Components?

Components are the building blocks of Yew. They manage their own state and can render themselves to the DOM. Components are created by implementing the `Component` trait which describes the lifecycle of a component.

## Lifecycle

:::important contribute
`Contribute to our docs:` [Add a diagram of the component lifecycle](https://github.com/yewstack/docs/issues/22)
:::

## Lifecycle Methods

### Create

When a component is created, it receives properties from its parent component as well as a `ComponentLink`. The properties can be used to initialize the component's state and the "link" can be used to register callbacks or send messages to the component.

It is common to store the props and the link in your component struct, like so:

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

Components declare their layout in the `view()` method. Yew provides the `html!` macro for declaring HTML and SVG nodes and their listeners as well as child components. The macro acts a lot like React's JSX, but uses Rust expressions instead of JavaScript.

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

For usage details, check out [the `html!` guide](../html/README.md):

### Rendered

The `rendered()` component lifecycle method is called after `view()` is processed and Yew has rendered your component, but before the browser refreshes the page. A component may wish to implement this method to perform actions that can only be done after the component has rendered elements. You can check whether this is the first time the component was rendered via the `first_render` parameter.

```rust
use stdweb::web::html_element::InputElement;
use stdweb::web::IHtmlElement;
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
            if let Some(input) = self.node_ref.try_into::<InputElement>() {
                input.focus();
            }
        }
    }
}
```

:::tip note
Note that this lifecycle method does not require an implementation and will do nothing by default
:::

### Update

Components are dynamic and can register to receive asynchronous messages. The `update()` lifecycle method is called for each message. This allows the component to update itself based on what the message was, and determine if it needs to re-render itself. Messages can be triggered by HTML elements listeners or be sent by child components, Agents, Services, or Futures.

Here's an example of what `update()` could look like:

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

Components may be re-rendered by their parents. When this happens, they could receive new properties and choose to re-render. This design facilitates parent to child component communication through changed properties.

A typical implementation would look like:

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

After Components are unmounted from the DOM, Yew calls the `destroy()` lifecycle method to support any necessary clean up operations. This method is optional and does nothing by default.

## Associated Types

The `Component` trait has two associated types: `Message` and `Properties`.

```rust
impl Component for MyComponent {
    type Message = Msg;
    type Properties = Props;

    // ...
}
```

`Message` represents a variety of messages that can be processed by the component to trigger some side effect. For example, you may have a `Click` message which triggers an API request or toggles the appearance of a UI component. It is common practice to create an enum called `Msg` in your component's module and use that as the message type in the component. It is common to shorten "message" to "msg".

```rust
enum Msg {
    Click,
}
```

`Properties` represents the information passed to a component from its parent. This type must implements the `Properties` trait \(usually by deriving it\) and can specify whether certain properties are required or optional. This type is used when creating and updating a component. It is common practice to create a struct called `Props` in your component's module and use that as the component's `Properties` type. It is common to shorten "properties" to "props". Since props are handed down from parent components, the root component of your application typically has a `Properties` type of `()`. If you wish to specify properties for your root component, use the `App::mount_with_props` method.

