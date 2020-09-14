# Two Apps Example

This example runs two Yew apps which communicate with each other.

## Concepts

The example illustrates how one can take control of the mounting process which is usually done by `yew::start_app`.

## Improvements

Instead of using the same component type twice, the example could use two entirely different components that communicate with each other.
One of the components could even accept a generic "remote" component using a trait.
