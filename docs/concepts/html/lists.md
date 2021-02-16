---
title: Lists
---

## Fragments

The `html!` macro always requires a single root node. In order to get around this restriction, you
can use an "empty tag" (these are also called "fragments" in the React world).

<!--DOCUSAURUS_CODE_TABS-->
<!--Valid-->
```rust
html! {
    <>
        <div></div>
        <p></p>
    </>
}
```

<!--Invalid-->
```rust
/* error: only one root html element allowed */

html! {
    <div></div>
    <p></p>
}
```
<!--END_DOCUSAURUS_CODE_TABS-->


## Iterators

Yew supports two different syntaxes for building HTML from an iterator.

The first is to call `collect::<Html>()` on the final transform in your iterator, which returns a
list that Yew can display.

<!--DOCUSAURUS_CODE_TABS-->
<!--Syntax Type 1-->
```rust
html! {
    <ul class="item-list">
        { self.props.items.iter().map(renderItem).collect::<Html>() }
    </ul>
}
```

The alternative is to use the `for` keyword, which is not native Rust syntax and instead is used by
the HTML macro to output the needed code to display the iterator.

<!--Syntax Type 2-->
```rust
html! {
    <ul class="item-list">
        { for self.props.items.iter().map(renderItem) }
    </ul>
}
```
<!--END_DOCUSAURUS_CODE_TABS-->

## Further reading
- [TodoMVC](https://github.com/yewstack/yew/tree/master/examples/todomvc)
- [Keyed list](https://github.com/yewstack/yew/tree/master/examples/keyed_list)
- [Router](https://github.com/yewstack/yew/tree/master/examples/router)
