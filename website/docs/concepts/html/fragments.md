---
title: "Fragments"
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

The `html!` macro always requires a single root node. In order to get around this restriction, you
can use an "empty tag" (these are also called "fragments").

<Tabs>
<TabItem value="Valid" label="Valid">

```rust
use yew::html;

html! {
    <>
        <div></div>
        <p></p>
    </>
};

```

</TabItem>

<TabItem value="Invalid" label="Invalid">

```rust, compile_fail
use yew::html;

// error: only one root html element allowed

html! {
    <div></div>
    <p></p>
};

```

</TabItem>
</Tabs>
