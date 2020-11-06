---
title: Functional Components
sidebar_label: Introduction
description: Introduction to functional components 
---

:::warning
Functional components are currently unreleased and should **not** be used today.
:::

:::important contribute
Contribute to our docs: Add details about functional components

Take a look at the [project board](https://github.com/yewstack/yew/projects/3) for details and consider helping out.
:::

Functional components are simply Rust functions which act like Yew components. 

## Creating functional components

Functional components are created using the [`#[functional_component]`](functional-components/macro.md) attribute.

### Hooks

Hooks are simply functions that let you “hook into” components' state and/or lifecycle and perform actions. Yew comes with a few pre-defined hooks. You can also create your own.

#### Pre-defined Hooks

Yew comes with the following predefined hooks:
- [`use_state`](functional-components/pre-defined-hooks.md#use_state)
- [`use_ref`](functional-components/pre-defined-hooks.md#use_ref)
- [`use_reducer`](functional-components/pre-defined-hooks.md#use_reducer)
- [`use_reducer_with_init`](functional-components/pre-defined-hooks.md#use_reducer_with_init)
- [`use_effect`](functional-components/pre-defined-hooks.md#use_effect)
- [`use_effect_with_deps`](functional-components/pre-defined-hooks.md#use_effect_with_deps)

#### Custom Hooks

There are cases where you want to define your own hooks for reasons. Yew allows you to define your own hooks which lets you extract your potentially stateful logic from the  component into reusable functions. 
