---
description: Parent to child communication
---

# Properties

As stated in the Components page, Properties are used to communicate from a parent to a child component.

## Derive macro

Don't try to implement `Properties` yourself, instead derive it by using `#[derive(Properties)]` instead.

### Required attribute

The fields within a struct that implements `Properties` must be either `Default` or have a `#[props(required)]` attribute applied to them. This attribute signals to the framework that the field must be supplied when the component is created in a `html!` macro, otherwise you will receive a compiler error. For fields that aren't required, it is common to wrap them in an `Option`, which will default to `None` when the field isn't supplied.

### PartialEq

It often makes sense to derive `PartialEq` on your props if you can. This makes it much easier to avoid rerendering using a trick explained in the **Optimizations & Best Practices** section.

## Memory/Speed overhead of Properties

Remember the component's `view` function signature:

```rust
fn view(&self) -> Html
```

You take a reference of the component's state, and use that to create `Html`. But properties are owned values. This means that in order to create them and pass them to child components, we need to get ownership of the references provided in the `view` function. This is done by implicitly cloning the references as they are passed to components in order to get owned values that constitute their props.

This means that each component has its own distinct copy of state passed down from its parent, and that whenever you re-render a component, the props for all child components of the re-rendering component will have to be cloned.

The implication of this is if you would otherwise be passing _huge_ amounts of data down as props \(Strings that are 10s of kilobytes in size\), you may want to consider turning your child component into a `Html`-returning function that runs in the parent, as you aren't forced to clone your data.

Alternatively, if you won't need to alter the large data that is passed as props, and only will display it, you can wrap it in an `Rc` so that only a ref-counted pointer is cloned,  instead of the data itself.

## Example

```rust
pub struct LinkColor {
    Blue,
    Red,
    Green
    Black,
    Purple,
}

impl Default for LinkColor {
    fn default() -> Self {
        // The link color will be Blue unless otherwise specified.
        LinkColor::Blue
    }
}

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    /// The link must have a target.
    #[props(required)]
    href: String,
    /// If the link text is huge, this will make copying the string much cheaper.
    /// This isn't usually recommended unless performance is a problem.
    #[props(required)]
    text: Rc<String>,
    /// Color of the link.
    color: LinkColor,
    /// The view function will not specify a size if this is None.
    size: Option<u32>
}
```
