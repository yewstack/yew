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
use yew::prelude::*;
use yew::html::ChildrenWithProps;

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

The [`derive_more`](https://github.com/JelteF/derive_more) crate is used here for better ergonomics. If you don't want
to use it, you can manually implement `From` for each variant.

```rust
use yew::prelude::*;
use yew::html::ChildrenRenderer;
use yew::virtual_dom::{ VChild, VComp };

// ...

#[derive(Clone, derive_more::From)]
pub enum ItemPropVariants {
	MyFirstComponent(MyFirstComponentProps),
	MySecondComponent(MySecondComponentProps),
}

#[derive(Clone)]
pub struct Item {
	props: ItemPropVariants,
}

impl<CHILD> From<VChild<CHILD>> for Item
where
	CHILD: Component,
	CHILD::Properties: Into<ItemPropVariants>,
{
	fn from(vchild: VChild<CHILD>) -> Self {
		Self {
			props: vchild.props.into(),
		}
	}
}

impl Into<Html> for Item {
	fn into(self) -> Html {
		match self.props {
			ItemPropVariants::MyFirstComponent(props) => {
				html! { <MyFirstComponent with props /> }
			}
			ItemPropVariants::MySecondComponent(props) => {
				html! { <MySecondComponent with props /> }
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
				{ for self.props.children.iter() }
			</div>
		}
	}
}
```

Heres a run down of whats happening:

This segment here defines the different types of properties that children
components can have. Additionally, it derives `derive_more::From`, which
implements `From<MyFirstComponentProps>` and `From<MySecondComponentProps>` for
us!

```rust
#[derive(Clone, derive_more::From)]
pub enum ItemPropVariants { /* ... */ }
```

Next, we create this wrapper:

```rust
#[derive(Clone)]
pub struct Item {
	props: ItemPropVariants,
}
```

Then, we implement `From<VChild<CHILD>>` for our wrapper, where `CHILD`:
 
 - Implements `Component`
 - Has a `Self::Properties` value that implements `Into<ItemPropVariants>`.

This tells Yew how to handle converting a virtual DOM child into our wrapper!

```rust
impl<CHILD> From<VChild<CHILD>> for Item
where
	CHILD: Component,
	CHILD::Properties: Into<ItemPropVariants>,
{ /* ... */ }
```

Finally, we implement `Into<Html>` for our wrapper, allowing it to be rendered!

```rust
impl Into<Html> for Item { /* ... */ }
```

### Sharing properties

This use case is mostly helpful for library developers, as you may want to
share state between parent and child components without causing difficulty for
users of your library.

```rust
// ... See the multiple typed children example ...

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
						item.props.value = format!("#{}", i);
						item
					})
				}
			</div>
		}
	}
}
```
