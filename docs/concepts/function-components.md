---
title: Function Components
sidebar_label: Introduction
description: Introduction to function components 
---

:::warning
Function components are currently unreleased and should **not** be used today.
:::

:::important contribute
Contribute to our docs: Add details about function components

Take a look at the [project board](https://github.com/yewstack/yew/projects/3) for details and consider helping out.
:::

Function components are simply Rust functions which act like Yew components. 

## Creating function components

function components are created using the [`#[function_component]`](function-components/macro.md) attribute.

### Hooks

Hooks are simply functions that let you “hook into” components' state and/or lifecycle and perform actions. Yew comes with a few pre-defined hooks. You can also create your own.

#### Pre-defined Hooks

Yew comes with the following predefined hooks:
- [`use_state`](function-components/pre-defined-hooks.md#use_state)
- [`use_ref`](function-components/pre-defined-hooks.md#use_ref)
- [`use_reducer`](function-components/pre-defined-hooks.md#use_reducer)
- [`use_reducer_with_init`](function-components/pre-defined-hooks.md#use_reducer_with_init)
- [`use_effect`](function-components/pre-defined-hooks.md#use_effect)
- [`use_effect_with_deps`](function-components/pre-defined-hooks.md#use_effect_with_deps)

#### Custom Hooks

There are cases where you want to define your own hooks for reasons. Yew allows you to define your own hooks which lets you extract your potentially stateful logic from the component into reusable functions. 
