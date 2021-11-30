---
title: "Classes"
description: "A handy macro to handle classes"
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

## Classes

The struct `Classes` can be used to deal with HTML classes.

When pushing a string to the set, `Classes` ensures that there is one element
for every class even if a single string might contain multiple classes.

`Classes` can also be merged by using `Extend` (i.e.
`classes1.extend(classes2)`) or `push()` (i.e. `classes1.push(classes2)`). In
fact, anything that implements `Into<Classes>` can be used to push new classes
to the set.

The macro `classes!` is a convenient macro that creates one single `Classes`.
Its input accepts a comma separated list of expressions. The only requirement
is that every expression implements `Into<Classes>`.

<Tabs>
  <TabItem value="Literal" label="Literal">

```rust
use yew::{classes, html};

html! {
    <div class={classes!("container")}></div>
};
```

  </TabItem>
  <TabItem value="Multiple" label="Multiple">

```rust
use yew::{classes, html};

html! {
  <div class={classes!("class-1", "class-2")}></div>
};
```

  </TabItem>
  <TabItem value="String" label="String">

```rust
use yew::{classes, html};

let my_classes = String::from("class-1 class-2");

html! {
  <div class={classes!(my_classes)}></div>
};
```

  </TabItem>
  <TabItem value="Optional" label="Optional">

```rust
use yew::{classes, html};

html! {
  <div class={classes!(Some("class"))} />
};
```

  </TabItem>
  <TabItem value="Vector" label="Vector">

```rust
use yew::{classes, html};

html! {
  <div class={classes!(vec!["class-1", "class-2"])}></div>
};
```

  </TabItem>
  <TabItem value="Array" label="Array">

```rust
use yew::{classes, html};

let my_classes = ["class-1", "class-2"];

html! {
  <div class={classes!(my_classes.as_ref())}></div>
};
```

  </TabItem>
</Tabs>

## Components that accept classes

```rust
use yew::{
    classes, html, Children, Classes, Component,
    Context, Html, Properties
};

#[derive(PartialEq, Properties)]
struct Props {
    #[prop_or_default]
    class: Classes,
    fill: bool,
    children: Children,
}

struct MyComponent;

impl Component for MyComponent {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            class,
            fill,
            children,
        } = &ctx.props();
        html! {
            <div
                class={classes!(
                    "my-container-class",
                    fill.then(|| Some("my-fill-class")),
                    class.clone(),
                )}
            >
                { children.clone() }
            </div>
        }
    }
}
```
