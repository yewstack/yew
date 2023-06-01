# TodoMVC Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Ffunction_todomvc)](https://examples.yew.rs/function_todomvc)

This is an implementation of [TodoMVC](http://todomvc.com/) for Yew using function components and hooks.

## Concepts

- Uses [`function_components`](https://yew.rs/docs/next/concepts/function-components/introduction)
- Uses [`gloo_storage`](https://docs.rs/gloo-storage/latest/gloo_storage/) to persist the state

## Improvements

- Use `yew-router` for the hash based routing
- Clean up the code

## Running

Run this application with the trunk development server:

```bash
trunk serve --open
```