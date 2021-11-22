---
title: "Fragments"
---

## Fragments

The `html!` macro always requires a single root node. In order to get around this restriction, you
can use an "empty tag" (these are also called "fragments").

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
