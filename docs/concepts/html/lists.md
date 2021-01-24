---
title: Lists
---

## Fragments

The `html!` macro always requires a single root node. In order to get around this restriction, it's valid to wrap content in empty tags:

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

Yew supports two different syntaxes for building html from an iterator:

<!--DOCUSAURUS_CODE_TABS-->
<!--Syntax Type 1-->
```rust
html! {
    <ul class="item-list">
        { self.props.items.iter().map(renderItem).collect::<Html>() }
    </ul>
}
```

<!--Syntax Type 2-->
```rust
html! {
    <ul class="item-list">
        { for self.props.items.iter().map(renderItem) }
    </ul>
}
```
<!--END_DOCUSAURUS_CODE_TABS-->

## Relevant examples
- [TodoMVC](https://github.com/yewstack/yew/tree/master/examples/todomvc)
- [Keyed List](https://github.com/yewstack/yew/tree/master/examples/keyed_list)
- [Router](https://github.com/yewstack/yew/tree/master/examples/router)
