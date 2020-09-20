---
title: How it works
description: Low level details about the framework
---

# Low-level library internals

## Under the hood of the `html!` macro

The `html!` macro turns code written in a custom HTML-like syntax into valid Rust code. Using this
macro is not necessary for developing Yew applications, but it is recommended. The code generated 
by this macro makes use of the public Yew library API which can be used directly if you wish. Note
that some methods used are undocumented intentionally to avoid accidental misuse. With each
update of `yew-macro`, the generated code will be more efficient and handle any breaking changes
without many (if any) modifications to the `html!` syntax.

Because the `html!` macro allows you to write code in a declarative style, your UI layout code will
closely match the HTML that is generated to the page. This becomes increasingly useful as your
application gets more interactive and your codebase gets larger. Rather than manually writing the
all of the code to manipulate the DOM yourself, the macro will handle it for you.

Using the `html!` macro can feel pretty magic, but it has nothing to hide. If you're curious about
how it works, try expanding the `html!` macro calls in your program. There's a useful command called
`cargo expand` which allows you to see the expansion of Rust macros. `cargo expand` isn't shipped with
`cargo` by default so you'll need to install it with `cargo install cargo-expand` if you haven't
already.

Note that when viewing expanded macro code, you're likely to encounter unusually verbose code. The
reason is because generated code can sometimes clash with other code in an application. In order
to prevent issues, `proc_macro` "hygiene" is adhered to. Some examples include:

1. Instead of using `yew::<module>` the macro generates `::yew::<module>` to make sure that the
Yew package is referenced correctly. This is also why `::alloc::vec::Vec::new()` is called instead
of just `Vec::new()`.
2. Due to potential trait method name collisions, `<Type as Trait>` is used to make sure that we're using items from the

## What is a virtual DOM?

The DOM ("document object model") is a representation of the HTML content that is managed by the browser
for your web page. A "virtual" DOM is simply a copy of the DOM that is held in application memory. Managing
a virtual DOM results in a higher memory overhead, but allows for batching and faster reads by avoiding
or delaying the use of browser APIs.

Having a copy of the DOM in memory can be really helpful for libraries which promote the use of
declarative UIs. Rather than needing specific code for describing how the DOM should be modified
in response to a user event, the library can use a generalized approach with DOM "diffing". When a Yew
component is updated and wants to change how it is rendered, the Yew library will build a second copy
of the virtual DOM and directly compare to a virtual DOM which mirrors what is currently on screen.
The "diff" (or difference) between the two can be broken down into incremental updates and applied in
a batch with browser APIs. Once the updates are applied, the old virtual DOM copy is discarded and the
new copy is saved for future diff checks.

This "diff" algorithm can be optimized over time to improve the performance of complex applications.
Since Yew applications are run with WebAssembly, we believe that Yew has a competitive edge to adopt
more sophisticated algorithms in the future.

The Yew virtual DOM is not exactly one-to-one with the browser DOM. It also includes "lists" and 
"components" for organizing DOM elements. A list can simply be an ordered list of elements but can
also be much more powerful. By annotating each list element with a "key", application developers
can help Yew make additional optimizations to ensure that when a list changes, the least amount
of work is done to calculate the diff update. Similarly, components provide custom logic to
indicate whether a re-render is required to help with performance.

## Yew scheduler and component-scoped event loop

*Contribute to the docs – explain how `yew::scheduler` and `yew::html::scope` work in depth*

## Further reading
* [More information about macros from the Rust Book](https://doc.rust-lang.org/stable/book/ch19-06-macros.html)
* [More information about `cargo-expand`](https://github.com/dtolnay/cargo-expand)
* [The API documentation for `yew::virtual_dom`](https://docs.rs/yew/*/yew/virtual_dom/index.html)