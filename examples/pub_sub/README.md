# Pub Sub Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Fpub_sub)](https://examples.yew.rs/pub_sub)

This is currently a technical demonstration of agents.

## Concepts

The example has two components, which communicate through a "broker" agent
as opposed to the traditional method using component links.

## Improvements

As it stands, this example uses a great amount of code to do very little.
The concept should be applied to a more elaborate use case.

This could also be merged into the [nested_list](../nested_list) example to remove the need for `WeakComponentLink`.
