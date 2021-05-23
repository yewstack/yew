---
title: Properties
description: Parent to child communication
id: version-0.17.3-properties
original_id: properties
---
Properties enable child and parent components to communicate with each other.

## Derive macro

Don't try to implement `Properties` yourself, derive it by using `#[derive(Properties)]` instead.

:::note
Types for which you derive `Properties` must also implement `Clone`. This can be done by either using `#[derive(Properties, Clone)]` or manually implementing `Clone` for your type.
:::

### Required attributes

The fields within a struct that derives `Properties` are required by default. When a field is missing and the component is created in the `html!` macro, a compiler error is returned. For fields with optional properties, use the `#[prop_or_default]` attribute to use the default value for that type when the prop is not specified. To specify a value, use the `#[prop_or(value)]` attribute where value is the default value for the property or alternatively use `#[prop_or_else(function)]` where `function` returns the default value. For example, to default a boolean value as `true`, use the attribute `#[prop_or(true)]`. It is common for optional properties to use the `Option` enum which has the default value `None`.

### PartialEq

It is likely to make sense to derive `PartialEq` on your props if you can do this. Using `PartialEq` makes it much easier to avoid unnecessary rerendering \(this is explained in the **Optimizations & Best Practices** section\).

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

impl Default for LinkColor {
    fn default() -> Self {
        // The link color will be blue unless otherwise specified.
        LinkColor::Blue
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps {
    /// The link must have a target.
    href: String,
    /// If the link text is huge, this will make copying the string much cheaper.
    /// This isn't usually recommended unless performance is known to be a problem.
    text: Rc<String>,
    /// Color of the link.
    #[prop_or_default]
    color: LinkColor,
    /// The view function will not specify a size if this is None.
    #[prop_or_default]
    size: Option<u32>,
    /// When the view function doesn't specify active, it defaults to true.
    #[prop_or(true)]
    active: bool,
}
```

