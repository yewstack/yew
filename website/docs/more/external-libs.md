---
title: "External libraries"
description: "Libraries that can help with Yew development"
---

### Yewtil

Yewtil is a collection of common utilities that help you build applications using Yew. It includes:

* NeqAssign - This is described in more detail in the section on 
[optimizations and best practices](../advanced-topics/optimizations.md) and ensures that identical
sets of props don't cause a component to re-render.

* PureComponents - Components that don't update any of their state. Using NeqAssign under the hood, they act as memoized 
  functions that are called from inside the `html!` macro like normal components are.

* Lrc - linked list reference counted smart pointer functions like `Rc` does, but allows for novel data update patterns.
* Mrc/Irc - Mutable/Immutable reference counted smart pointers that function like `Rc` but are more ergonomic to use 

  within Yew, because they implement `DerefMut` and `BorrowMut`for `Mrc`. This allows `Mrc` to be used with `NeqAssign`. 

  `Irc` acts as an immutable view of the data, which makes this ideal for holding data used in display-only tasks.

* History - A history tracking wrapper that uses a `VecDeque` to hold on to previous values that it 
has represented.
* Futures - Support for running futures that send messages to component update loops.
* Fetch - A wrapper around `web_sys` to make HTTP requests.

## Malvolio

[Malvolio](https://crates.io/crates/malvolio) is a library with a "builder-syntax" for creating complex HTML documents 
with ease. It runs both on servers (and renders to strings) or in browsers (with Yew).

## Weblog

[weblog](https://crates.io/crates/weblog) is a crate that defines a set of macros for calling `console.log()`,
`console.error()` and other members of the browser's console API when targeting WASM.

## Gloo

[Gloo](https:://crates.io/crates/gloo) is a modular toolkit for building fast, reliable Web applications and
libraries with Rust and Wasm. Gloo provides ergonomic Rust APIs for working with:

- [Console timers](https://crates.io/crates/gloo-console-timer)
- [Dialogs](https://crates.io/crates/gloo-dialogs)
- [Events](https://crates.io/crates/gloo-events)
- [Files](https://crates.io/crates/gloo-file)
- [Timers](https://crates.io/crates/gloo-timers)
- [Web Storage](https://crates.io/crates/gloo-storage)

## Reqwasm

[Reqwasm](https://crates.io/crates/reqwasm) is an HTTP requests library for WASM Apps.
It provides idiomatic Rust API for the browser's `fetch` and `WebSocket` API.

## Looking For

Libraries that the ecosystem needs, but doesn't have yet.

Bootstrap/MaterialUi/arbitrary css framework component wrappers.
