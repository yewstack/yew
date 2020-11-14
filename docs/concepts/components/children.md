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

There are some cases where you wouldn't want to use the `Children` type
and instead use an alternative method:

 - You want to only allow components of a specific type to be used as children for your component.
 - You want to share state between a component and its children.

### Typed children

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

### Sharing properties

This use case is mostly helpful for library developers, as you may want to
share state between parent and child components without causing difficulty for
users of your library.

```rust
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::{VChild, VComp};

// This enum defines the different types of properties that children components
// can have. Additionally, it derives `derive_more::From`, which implements 
// `From<PrimaryProps>` and `From<SecondaryProps>` for us!
#[derive(Clone, derive_more::From)]
pub enum ItemProps {
    Primary(PrimaryProps),
    Secondary(SecondaryProps),
}

// This struct acts as a simple wrapper ...
#[derive(Clone)]
pub struct Item {
    props: ItemProps,
}

// ... which we implement `From<VChild<CHILD>>` for, where `CHILD`:
//
// - Implements `Component`
// - Has a `Self::Properties` value that implements `Into<ItemProps>`.
// 
// This tells Yew how to handle converting a virtual DOM child into our
// wrapper!
impl<CHILD> From<VChild<CHILD>> for Item
where
    CHILD: Component,
    CHILD::Properties: Into<ItemProps>,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: vchild.props.into(),
        }
    }
}

// Finally, we implement `Into<Html>` for our wrapper, allowing it to be
// rendered!
impl Into<Html> for Item {
    fn into(self) -> Html {
        match self.props {
            ItemProps::Primary(props) => {
                html! { <Primary with props /> }
            }
            ItemProps::Secondary(props) => {
                html! { <Secondary with props /> }
            }
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
                {
                    for self.props.children.iter().enumerate().map(|(i, mut item)| {
                        match item.props {
                            ItemProps::Primary(mut props) => {
                                props.label = format!("#{:02}", i);
                            }
                            ItemProps::Secondary(mut props) => {
                                props.description = format!("This is child #{:02}.", i);
                            }
                        }
                        item
                    })
                }
            </div>
        }
    }
}
```
