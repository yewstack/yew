---
title: Children
description: 
---

## General usage

_Most of the time,_ when allowing a component to have children, you don't care 
what type of children the component has. In such cases, the below example will
suffice.

```rust
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct ListProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct List {
    props: ListProps,
}

impl Component for List {
    type Properties = ListProps;
    // ...

    fn view(&self) -> Html {
        html! {
            <div class="list">
                { for self.props.children.iter() }
            </div>
        }
    }
}
```

## Advanced usage

In cases where you want one type of component to be passed as children to your component,
you can use `yew::html::ChildrenWithProps<T>`.

```rust
use yew::html::ChildrenWithProps;
use yew::prelude::*;

// ...

#[derive(Properties, Clone)]
pub struct ListProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<Item>,
}

pub struct List {
    props: ListProps,
}

impl Component for ListProps {
    type Properties = ListProps;
    // ...

    fn view(&self) -> Html {
        html! {
            <div class="list">
                { for self.props.children.iter() }
            </div>
        }
    }
}
```

Of course, sometimes you might need to restrict the children to a few different
components. In these cases, you have to get a little more hands-on with Yew.

The [`derive_more`](https://github.com/JelteF/derive_more) crate is used here
for better ergonomics. If you don't want to use it, you can manually implement
`From` for each variant.

```rust
use yew::prelude::*;
use yew::html::ChildrenRenderer;
use yew::virtual_dom::{ VChild, VComp };

// `derive_more::From` implements `From<VChild<Primary>>` and
// `From<VChild<Secondary>>` for `Item` automatically!
#[derive(Clone, derive_more::From)]
pub enum Item {
    Primary(VChild<Primary>),
    Secondary(VChild<Secondary>),
}

// Now, we implment `Into<Html>` so that yew knows how to render `Item`.
impl Into<Html> for Item {
    fn into(self) -> Html {
        match self {
            Self::Primary(child) => child.into(),
            Self::Secondary(child) => child.into(),
        }
    }
}

#[derive(Properties, Clone)]
pub struct ListProps {
    #[prop_or_default]
    pub children: ChildrenRenderer<Item>,
}

pub struct List {
    props: ListProps,
}

impl Component for List {
    type Properties = ListProps;
    // ...

    fn view(&self) -> Html {
        html! {
            <div class="list">
                { for self.props.children.iter() }
            </div>
        }
    }
}
```

