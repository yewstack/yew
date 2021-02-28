# TodoMVC Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Ftodomvc)](https://examples.yew.rs/todomvc)

This is an implementation of [TodoMVC](http://todomvc.com/) for Yew.

Unlike other implementations, this stores the full state of the model,
including: all entries, entered text and chosen filter.

## Concepts

- Uses [`StorageService`] to persist the state
- [`Refs`] are used to manipulate DOM elements after they're rendered (to automatically focus input fields for instance)

## Improvements

- Use `yew-router` for the hash based routing
- Clean up the code

[`storageservice`]: https://docs.rs/yew-services/latest/yew_services/struct.StorageService.html
[`refs`]: https://yew.rs/docs/en/concepts/components/refs/
