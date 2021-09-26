# TodoMVC Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Ffunction_todomvc)](https://examples.yew.rs/function_todomvc)

This is an implementation of [TodoMVC](http://todomvc.com/) for Yew using function components and hooks.

## Concepts

- Uses [`function_components`](https://yew.rs/next/concepts/function-components)
- Uses [`gloo_storage`](https://gloo-rs.web.app/docs/storage) to persist the state
- [`Refs`] are used to manipulate DOM elements after they're rendered (to automatically focus input fields for instance)

## Improvements

- Use `yew-router` for the hash based routing
- Clean up the code

[`refs`]: https://yew.rs/concepts/components/refs/
