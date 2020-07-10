---
description: Libraries that can help with yew development
---

# External Libs

### Yewtil

Yewtil is a collection of common utilities that help you write Yew programs. It includes:

* NeqAssign - As discussed earlier, is the best way to assign props to ensure minimal re-rendering.
* PureComponents - Components that don't update any of their state. Using NeqAssign under the hood, they act as memoized 

  functions that are called from inside the `html!` macro like normal components are.

* Lrc - linked list reference counted smart pointer functions like `Rc` does, but allows for novel data update patterns.
* Mrc/Irc - Mutable/Immutable reference counted smart pointers that function like `Rc` but are more ergonomic to use 

  within Yew, due to implementing `DerefMut` and `BorrowMut`for `Mrc`. This allows `Mrc` to be used with `NeqAssign`. 

  `Irc` acts as an immutable view into the data, which makes this ideal for holding data used in display-only tasks.

* History - A history tracking wrapper that uses a `VecDeque` to hold on to previous values that it has represented.
* Futures - Support for running futures that send messages to component update loops.
* Fetch - Abstractions for handling fetch requests made using `web_sys` and the aforementioned futures feature.

## Looking For

Libraries that the ecosystem needs, but doesn't have yet.

Bootstrap/MaterialUi/arbitrary css framework component wrappers.

