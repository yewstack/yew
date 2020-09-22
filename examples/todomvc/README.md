# TodoMVC Example

This is an implementation of [TodoMVC](http://todomvc.com/) for Yew.

Unlike other implementations, this stores the full state of the model,
including: all entries, entered text and chosen filter.

## Concepts

- Uses [`StorageService`] to persist the state
- [`Refs`] are used to manipulate DOM elements after they're rendered (to automatically focus input fields for instance)

## Improvements

- Use `yew-router` for the filters

[`storageservice`]: https://docs.rs/yew/latest/yew/services/struct.StorageService.html
[`refs`]: https://yew.rs/docs/en/concepts/components/refs/
