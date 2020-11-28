---
title: Function Components
sidebar_label: Introduction
description: Introduction to function components 
---

:::warning
We're still working on function components and Hooks. They're not quite ready to be used yet.
If you'd like to help out, take a look at the [project board](https://github.com/yewstack/yew/projects/3) for a list of things that still need to be done.
:::


Function components are a simplified version of normal components.
They consist of a single function that receives props and determines what should be rendered by returning `Html`.
Basically, it's a component that's been reduced to just the `view` method.
On its own that would be quite limiting because you can only create pure components, but that's where Hooks come in.
Hooks allow function components to use state and other Yew features without implementing the `Component` trait.

## Creating function components

The easiest way to create a function component is to add the [`#[function_component]`](function-components/attribute.md) attribute to a function.

```rust
#[function_component(HelloWorld)]
fn hello_world() -> Html {
    html! { "Hello world" }
}
```

### Under the hood

Function components consists of two parts.
First, the `FunctionProvider` trait which is comparable to the `Component` trait but it only has a single method called `run`.
The second part is the `FunctionComponent` struct which wraps around the `FunctionProvider` type and turns it into an actual `Component`. 
The `#[function_component]` attribute essentially just implements `FunctionProvider` for you and exposes it wrapped in `FunctionComponent`.

### Hooks

Hooks are simply functions that let you “hook into” components' state and/or lifecycle and perform actions. Yew comes with a few pre-defined Hooks. You can also create your own.

#### Pre-defined Hooks

Yew comes with the following predefined Hooks:
- [`use_state`](function-components/pre-defined-hooks.md#use_state)
- [`use_ref`](function-components/pre-defined-hooks.md#use_ref)
- [`use_reducer`](function-components/pre-defined-hooks.md#use_reducer)
- [`use_reducer_with_init`](function-components/pre-defined-hooks.md#use_reducer_with_init)
- [`use_effect`](function-components/pre-defined-hooks.md#use_effect)
- [`use_effect_with_deps`](function-components/pre-defined-hooks.md#use_effect_with_deps)

#### Custom Hooks

There are cases where you want to define your own Hooks for reasons. Yew allows you to define your own Hooks which lets you extract your potentially stateful logic from the component into reusable functions. 
See the [Defining custom hooks](function-components/custom-hooks.md#defining-custom-hooks) section for more information.
