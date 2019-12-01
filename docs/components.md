---
description: Components and their lifecycle hooks
---

# Components

## What are Components?

Components are the building blocks of Yew and contain their own state and can update and render themselves to the DOM.

`Component` is probably the most prominent trait in Yew. It describes the lifecycle of a component attached to the DOM. What happens when it is created, how it updates, how it renders, and more, is described by this trait.

A struct that implements `Component` can be used in the `html!` macro in a variety of ways:

```rust
html!{
    <>
        // Alone (or with non-required props)
        <MyComponent />

        // With children (if the props for MyComponent have the field - children: Children<MyComponent>)
        <MyComponent>
            <div> {"Arbitrary Content"} </div>
        </MyComponent>

        // With Properties
        <MyComponentWithProps prop1 = "lorem" prop2 = "ipsum" />

        // With the whole set of props provided at once
        <MyComponentWithProps with props />
    </>
}
```

## Associated Types

The `Component` trait has two associated types: `Message` and `Properties`.

`Message` is typically an enum, although it can be a struct, that represents a variety of messages that can be processed by the component. It is common practice to create a type called `Msg` or `ComponentNameMsg` in your component's module and use that as the message type in the component. It is common to shorten "message" to "msg".

The `Properties` associated type is a struct that has implemented the `Properties` trait \(usually by deriving it\). This type is used when creating and updating a component. It is common practice to create an enum called `Props` or `ComponentNameProps` in your component's module and use that as the component's `Properties` type. It is common to shorten "properties" to "props". Since props are handed down from parent components, the root component of your application typically has a `Properties` type of `()`, because it has no state to inherit from its parent.

## Component Lifecycle

### Create

When a component is created, it gets a copy of the component's properties, and a link, and is expected to produce an instance of the component's model - the model being the state that makes up the component.

Properties come from the parent and should be used to initialize your component to a particular state, or be persisted in its entirety. The `ComponentLink<Self>` item is used to register callbacks or send messages to the component. It is useful to keep around for use in the `update()` method.

It is common to keep all of the props, and the link as well for medium to large sized components. In these cases its recommended to structure the model you are implementing `Component` for like so:

```rust
pub struct MyComponent {
    /// State from the parent
    props: MyComponentProps,
    /// Utility object
    link: ComponentLink<Link>,
    /// State that is created during the lifecycle of the component
    input_box_text: String,
    // ...
}
```

And implement `create()` like so:

```rust
impl Component for MyComponent {
    // ...
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MyComponent {
            props,
            link,
            ..Default::default() // Fill the remaining fields with their default values.
        }
    }
    // ...
}
```

### Mounted

`create()` initializes your component's state, but it isn't mounted to the DOM yet. `mounted()` is called directly after the component has been mounted to the DOM, which allows you perform actions that should be done after the component has rendered once. `mounted()` does not have to be implemented, and by default it will do nothing.

### Update

When a component receives a message, its `update()` method is called. This allows the component to update itself based on what the message was, and determine if it needs to re-render. Messages can come from HTML elements, child components, or Agents.

An example of update would be:

```rust
pub enum Msg {
    SetInputBoxText(String)
}

impl Component for MyComponent {
    // ...
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
       match msg {
           Msg::SetInputBoxText(text) => {
               self.input_box_text = text;
               true // Re-render when the input box text is set.
           }
       }
    }
    // ...
}
```

### Change

`change()` is like `update()` but it handles communications from its parent component. This is done in the form of its props being changed. You don't have to implement `change()` but you probably want to if you want to update a component via props after it has been created.

A naive implementation would look like:

```rust
impl Component for MyComponent {
    // ...
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
       self.props = props;
       true // This will always re-render when new props are provided.
    }
    // ...
}
```

### Destroy

Its called right before the component is unmounted from the DOM.

### View

`view()` is where you take a reference to the component's model, and create `Html<Self>` using the `html!` macro. `html!` has its own section, but in short, it acts like JSX does for React, embedding a HTML-alike language inside of Rust.

TODO example

## Nested Components

TODO

## Full example

A complete example for the snippets code can be found here.

