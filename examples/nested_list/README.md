# Nested List Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Fnested_list)](https://examples.yew.rs/nested_list)

This example shows a nested list and displays which item was last hovered.

## Concepts

- Creating components which only accepts specific child elements
- Communicating with a component that isn't a parent. See `WeakComponentLink` in [main.rs](src/main.rs).

## Improvements

- `ListItem` Component has a `hide` prop which is currently only used statically.
  It should be possible to make the hidden items visible by pressing a button.

## Running

Run this application with the trunk development server:

```bash
trunk serve --open
```