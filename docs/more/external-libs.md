---
title: External libraries
description: Libraries that can help with Yew development
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

## Looking For

Libraries that the ecosystem needs, but doesn't have yet.

Bootstrap/MaterialUi/arbitrary css framework component wrappers.
