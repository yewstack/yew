---
title: Refs
description: Out-of-band DOM access
id: version-0.17.3-refs
original_id: refs
---

The `ref` keyword can be used inside of any HTML element or component to get the DOM `Element` that the item is attached to. This can be used to make changes to the DOM outside of the `view` lifecycle method.

This is useful for getting ahold of canvas elements, or scrolling to different sections of a page.

The syntax is:

```rust
// In create
self.node_ref = NodeRef::default();

// In view
html! {
    <div ref={self.node_ref.clone()}></div>
}

// In update
let has_attributes = self.node_ref.cast::<Element>().unwrap().has_attributes();
```
