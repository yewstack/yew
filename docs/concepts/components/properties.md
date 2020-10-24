---
title: Properties
description: Parent to child communication
---

Properties enable child and parent components to communicate with each other.
Every component has an associated properties type which describes what is passed down from the parent.
In theory this can be any type that implements the `Properties` trait, but in practice there's no
reason for it to be anything but a struct where each field represents a property.

## Derive macro

Instead of implementing the `Properties` trait yourself, you should use `#[derive(Properties)]` to
automatically generate the implementation instead.
Types for which you derive `Properties` must also implement `Clone`.

### Field attributes

When deriving `Properties`, all fields are required by default.
The following attributes allow you to give your props initial values which will be used unless they're set to another value.

:::tip
Attributes aren't visible in Rustdoc generated documentation.
The docstrings of your properties should mention whether a prop is optional and if it has a special default value.
:::

#### `#[prop_or_default]`

Initialize the prop value with the default value of the field's type using the `Default` trait.

#### `#[prop_or(value)]`

Use `value` to initialize the prop value. `value` can be any expression that returns the field's type.
For example, to default a boolean prop to `true`, use the attribute `#[prop_or(true)]`.

#### `#[prop_or_else(function)]`

Call `function` to initialize the prop value. `function` should have the signature `FnMut() -> T` where `T` is the field type.

:::warning
The function is currently called even if the prop is explicitly set. If your function is performance intensive, consider using `Option` where `None` values are initialized in the `create` method.
See [#1623](https://github.com/yewstack/yew/issues/1623)
:::

## PartialEq

It makes sense to derive `PartialEq` on your props if you can do so.
Using `PartialEq` makes it much easier to avoid unnecessary rendering \(this is explained in the **Optimizations & Best Practices** section\).

## Memory/speed overhead of using Properties

In `Component::view`, you take a reference to the component's state, and use that to create `Html`. Properties, however, are owned values. This means that in order to create them and pass them to child components, we need to take ownership of the references provided in the `view` function. This is done by implicitly cloning the references as they are passed to components in order to get owned values.

This means that each component has its own distinct copy of the state passed down from its parent, and that whenever you re-render a component, the props for all child components of the re-rendering component will have to be cloned.

The implication of this is if you would otherwise be passing _huge_ amounts of data down as props \(Strings that are 10s of kilobytes in size\), you may want to consider turning your child component into a function which returns `Html` that the parent calls, as this means that data does not have to be cloned.

If you won't need to modify the data passed down through props you can wrap it in an `Rc` so that only a reference-counted pointer to the data is cloned, instead of the actual data itself.

## Example

```rust
use std::rc::Rc;
use yew::Properties;

#[derive(Clone, PartialEq)]
pub enum LinkColor {
    Blue,
    Red,
    Green,
    Black,
    Purple,
}

fn create_default_link_color() -> LinkColor {
    LinkColor::Blue
}

#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps {
    /// The link must have a target.
    href: String,
    /// If the link text is huge, this will make copying the string much cheaper.
    /// This isn't usually recommended unless performance is known to be a problem.
    text: Rc<str>,
    /// Color of the link. Defaults to `Blue`.
    #[prop_or_else(create_default_link_color)]
    color: LinkColor,
    /// The view function will not specify a size if this is None.
    #[prop_or_default]
    size: Option<u32>,
    /// When the view function doesn't specify active, it defaults to true.
    #[prop_or(true)]
    active: bool,
}
```

## Props macro

The `yew::props!` macro allows you to build properties the same way the `html!` macro does it.

The macro uses the same syntax as a struct expression except that you can't use attributes or a base expression (`Foo { ..base }`).
The type path can either point to the props directly (`path::to::Props`) or the associated properties of a component (`MyComp::Properties`).

```rust
let props = yew::props!(LinkProps {
    href: "/",
    text: Rc::from("imagine this text being really long"),
    size: 64,
});

// build the associated properties of a component
let props = yew::props!(Model::Properties {
    href: "/book",
    text: Rc::from("my bestselling novel"),
});
```
