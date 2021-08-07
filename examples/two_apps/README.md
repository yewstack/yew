# Two Apps Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Ftwo_apps)](https://examples.yew.rs/two_apps)

This example runs two Yew apps which communicate with each other.

## Concepts

The example illustrates how one can take control of the mounting process which is usually done by `yew::start_app`.

## Improvements

Instead of using the same component type twice, the example could use two entirely different components that communicate with each other.
One of the components could even accept a generic "remote" component using a trait.

This example is very similar to [`mount_point`](../mount_point).
The two should be merged into a single example.
