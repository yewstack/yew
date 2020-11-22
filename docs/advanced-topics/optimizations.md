---
title: Optimizations & Best Practices
sidebar_label: Optimizations
description: Make your app faster
---

## neq\_assign

When a component receives props from its parent component, the `change` method is called. This, in 
addition to allowing you to update the component's state, also allows you to return a `ShouldRender` 
boolean value that indicates if the component should re-render itself in response to the prop changes.

Re-rendering is expensive, and if you can avoid it, you should. As a general rule, you only want to 
re-render when the props actually changed. The following block of code represents this rule, returning 
`true` if the props differed from the previous props:

```rust
use yew::ShouldRender;

#[derive(PartialEq)]
struct ExampleProps;

struct Example {
    props: ExampleProps,
};

impl Example {
    fn change(&mut self, props: ExampleProps) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }
}
```

But we can go further! This is six lines of boilerplate can be reduced down to one by using a trait 
and a blanket implementation for anything that implements `PartialEq`. Check out the [`yewtil` 
crate's `NeqAssign` trait](https://docs.rs/yewtil/*/yewtil/trait.NeqAssign.html) which implements
this.

## Using smart pointers effectively

**Note: if you're unsure about some of the terms used in this section, the Rust Book has a useful
[chapter about smart pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html).** 

In an effort to avoid cloning large amounts of data to create props when re-rendering, we can use 
smart pointers to only clone a reference to the data instead of the data itself. If you pass 
references to the relevant data in your props and child components instead of the actual data you 
can avoid cloning any data until you need to modify it in the child component, where you can 
use `Rc::make_mut` to clone and obtain a mutable reference to the data you want to alter. 

This brings further benefits in `Component::change` when working out whether prop changes require 
the component to re-render. This is because instead of comparing the value of the data the 
underlying pointer addresses (i.e. the position in a machine's memory where the data is stored) can 
instead be compared; if two pointers point to the same data then the value of the data they point to 
must be the same. Note that the inverse might not be true! Even if two pointer addresses differ the 
underlying data might still be the same - in this case you should compare the underlying data.

To do this comparison you'll need to use `Rc::ptr_eq` instead of just using `PartialEq` (which is
automatically used when comparing data using the equality operator `==`). The Rust documentation 
has [more details about `Rc::ptr_eq`](https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#method.ptr_eq).

This optimization is most useful for data types that don't implement `Copy`. If you can copy your 
data cheaply, then it isn't worth putting it behind a smart pointer. For structures that 
can be data-heavy like `Vec`s, `HashMap`s, and `String`s using smart pointers is likely to bring
performance improvements.

This optimization works best if the values are never updated by the children, and even better, if 
they are rarely updated by parents. This makes `Rc<_>s` a good choice for wrapping property values 
in for pure components.

## View functions

For code readability reasons, it often makes sense to migrate sections of `html!` to their own 
functions. Not only does this make your code more readable because it reduces the amount of 
indentation present, it also encourages good design patterns – particularly around building
composable applications because these functions can be called in multiple places which reduces the
amount of code that has to be written. 

## Pure Components

Pure components are components that don't mutate their state, only displaying content and 
propagating messages up to normal, mutable components. They differ from view functions in that they 
can be used from within the `html!` macro using the component syntax \(`<SomePureComponent />`\) 
instead of expression syntax \(`{some_view_function()}`\), and that depending on their 
implementation, they can be memoized (this means that once a function is called its value is "saved"
so that if it's called with the same arguments more than once it doesn't have to recompute its value
and can just return the saved value from the first function call) - preventing re-renders for 
identical props using the aforementioned `neq_assign` logic.

Yew doesn't natively support pure or function components, but they are available via external crates.

## Keyed DOM nodes when they arrive

## Reducing compile time using workspaces

Arguably, the largest drawback to using Yew is the long time it takes to compile Yew apps. The time 
taken to compile a project seems to be related to the quantity of code passed to the `html!` macro. 
This tends to not be much of an issue for smaller projects, but for larger applications it makes 
sense to split code across multiple crates to minimize the amount of work the compiler has to do for 
each change made to the application.

One possible approach is to make your main crate handle routing/page selection, and then make a 
different crate for each page, where each page could be a different component, or just a big 
function that produces `Html`. Code which is shared between the crates containing different parts of
the application could be stored in a separate crate which is depended on throughout the project.
In the best case scenario, you go from rebuilding all of your code on each compile to rebuilding 
only the main crate, and one of your page crates. In the worst case, where you edit something in the 
"common" crate, you will be right back to where you started: compiling all code that depends on that 
commonly shared crate, which is probably everything else.

If your main crate is too heavyweight, or you want to rapidly iterate on a deeply nested page \(eg. 
a page that renders on top of another page\), you can use an example crate to create a simplified 
implementation of the main page and render the component you are working on on top of that.

## Reducing binary sizes

* optimize Rust code
  * `wee_alloc` \( using tiny allocator \)
  * `cargo.toml` \( defining release profile \)
* optimize wasm code using `wasm-opt`

**Note: more information about reducing binary sizes can be found in the 
[Rust Wasm Book](https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size).**

### wee\_alloc

[wee\_alloc](https://github.com/rustwasm/wee_alloc) is a tiny allocator that is much smaller than the allocator that is normally used in Rust binaries. Replacing the default allocator with this one will result in smaller Wasm file sizes, at the expense of speed and memory overhead.

The slower speed and memory overhead are minor in comparison to the size gains made by not including the default allocator. This smaller file size means that your page will load faster, and so it is generally recommended that you use this allocator over the default, unless your app is doing some allocation-heavy work.

```rust
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

### Cargo.toml

It is possible to configure release builds to be smaller using the available settings in the 
`[profile.release]` section of your `Cargo.toml`.


```text
[profile.release]
# less code to include into binary
panic = 'abort' 
# optimization over all codebase ( better optimization, slower build )
codegen-units = 1
# optimization for size ( more aggressive )
opt-level = 'z' 
# optimization for size 
# opt-level = 's' 
# link time optimization using using whole-program analysis
lto = true
```

### wasm-opt

Further more it is possible to optimize size of `wasm` code.

The Rust Wasm Book has a section about reducing the size of Wasm binaries:
[Shrinking .wasm size](https://rustwasm.github.io/book/game-of-life/code-size.html)

* using `wasm-pack` which by default optimizes `wasm` code in release builds
* using `wasm-opt` directly on `wasm` files.

```text
wasm-opt wasm_bg.wasm -Os -o wasm_bg_opt.wasm
```

#### Build size of 'minimal' example in yew/examples/

Note: `wasm-pack` combines optimization for Rust and Wasm code. `wasm-bindgen` is used in this example without any Rust size optimization.

| used tool | size |
| :--- | :--- |
| wasm-bindgen | 158KB |
| wasm-bindgen + wasm-opt -Os | 116KB |
| wasm-pack | 99 KB |

## Further reading:
 * [The Rust Book's chapter on smart pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
 * [Information from the Rust Wasm Book about reducing binary sizes](https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size)
 * [Documentation about Rust profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
 * [binaryen project](https://github.com/WebAssembly/binaryen)
