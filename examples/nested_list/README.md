# Nested List Example

This example shows a nested list and displays which item was last hovered.

## Concepts

- Creating components which only accepts specific child elements
- Communicating with a component that isn't a parent. See `WeakComponentLink` in [main.rs](src/main.rs).

## Improvements

- `ListItem` Component has a `hide` prop which is currently only used statically.
  It should be possible to make the hidden items visible by pressing a button.
