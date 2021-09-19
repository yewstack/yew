---
title: "Lists"
---

## Fragments

The `html!` macro always requires a single root node. In order to get around this restriction, it's valid to wrap content in empty tags:

<!--DOCUSAURUS_CODE_TABS-->
<!--Valid-->
```rust
use yew::html;

html! {
    <>
        <div></div>
        <p></p>
    </>
};
```

<!--Invalid-->
```rust ,compile_fail
use yew::html;

/* error: only one root html element allowed */

html! {
    <div></div>
    <p></p>
};
```
<!--END_DOCUSAURUS_CODE_TABS-->


## Iterators

Yew supports two different syntaxes for building html from an iterator:

<!--DOCUSAURUS_CODE_TABS-->
<!--Syntax Type 1-->
```rust
use yew::{html, Html};

let items = (1..=10).collect::<Vec<_>>();

html! {
    <ul class="item-list">
        { items.iter().collect::<Html>() }
    </ul>
};
```

<!--Syntax Type 2-->
```rust
use yew::{html};

let items = (1..=10).collect::<Vec<_>>();

html! {
    <ul class="item-list">
        { for items.iter() }
    </ul>
};
```
<!--END_DOCUSAURUS_CODE_TABS-->

## Relevant examples
- [TodoMVC](https://github.com/yewstack/yew/tree/master/examples/todomvc)
- [Keyed List](https://github.com/yewstack/yew/tree/master/examples/keyed_list)
- [Router](https://github.com/yewstack/yew/tree/master/examples/router)
