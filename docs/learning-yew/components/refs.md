---
description: Out-of-band DOM access
---

# Refs

## Refs

The `ref` keyword can be used inside of any HTML element or component to get the dom `Element` that the item is attached to. This can be used to make changes to the DOM outside of the `view` hook. 

This is useful for getting ahold of canvas elements, or telling divs to scroll to the bottom.

The syntax is:

```rust
// In create
self.node_ref = NodeRef::default();

// In view
html! {
    <div ref = self.node_ref.clone() ></div>
}

// In update
let has_attributes = self.node_ref.try_into::<Element>().has_attributes();
```



